//! Support for the PMSAv8-64 EL1 and EL2 MPUs

use tock_registers::interfaces::{Readable, Writeable};

use crate::{asm::barrier, registers};

#[doc(inline)]
pub use registers::PRBAR_EL1::{AP::Value as El1AccessPerms, SH::Value as El1Shareability};

#[doc(inline)]
pub use registers::PRBAR_EL2::{AP::Value as El2AccessPerms, SH::Value as El2Shareability};

/// Ways this API can fail
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Found too many regions
    ///
    /// Reports how many regions you tried to use. This number will be higher than
    /// `mpu.num_regions()`.
    TooManyRegions(usize),
    /// Found an invalid MAIR selector (only 0..=7 is valid)
    InvalidMair(u8),
    /// Found a region with invalid alignment
    UnalignedRegion(core::ops::RangeInclusive<*const u8>),
}

/// Represents our PMSAv8-32 EL1 MPU
pub struct El1Mpu();

impl El1Mpu {
    /// Create an MPU handle
    ///
    /// # Safety
    ///
    /// Only create one of these at any given time, as they access shared
    /// mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> El1Mpu {
        El1Mpu()
    }

    /// How many MPU regions are there?
    pub fn num_regions(&self) -> u8 {
        registers::MPUIR_EL1.read(registers::MPUIR_EL1::Regions) as u8
    }

    /// Access the current state of a region
    pub fn get_region(&mut self, idx: u8) -> Option<El1Region> {
        if idx >= self.num_regions() {
            return None;
        }
        registers::PRSELR_EL1.write(registers::PRSELR_EL1::Regions.val(idx as u64));
        let prbar = registers::PRBAR_EL1.extract();
        let prlar = registers::PRLAR_EL1.extract();
        let start_addr = (prbar.read(registers::PRBAR_EL1::Base) << 6) as *const u8;
        let end_addr = ((prlar.read(registers::PRLAR_EL1::Limit) << 6) | 0x3F) as *const u8;
        let shareability = prbar.read_as_enum(registers::PRBAR_EL1::SH)?;
        let access = prbar.read_as_enum(registers::PRBAR_EL1::AP)?;
        let no_exec = prbar
            .read_as_enum::<registers::PRBAR_EL1::NX::Value>(registers::PRBAR_EL1::NX)?
            == registers::PRBAR_EL1::NX::Value::ExecutionNotPermitted;
        let mair = prlar.read(registers::PRLAR_EL1::AttrIndex) as u8;
        let enable = prlar
            .read_as_enum::<registers::PRLAR_EL1::En::Value>(registers::PRLAR_EL1::En)?
            == registers::PRLAR_EL1::En::Value::Enabled;
        Some(El1Region {
            range: start_addr..=end_addr,
            shareability,
            access,
            no_exec,
            mair,
            enable,
        })
    }

    /// Write a single region to the EL1 MPU
    ///
    /// ## Arguments
    ///
    /// - `region`: The [El1Region] object containing the configuration for the MPU region.
    /// - `idx`: The index of the region to be configured.
    ///
    /// ## Errors
    ///
    /// Returns:
    /// - [Error::UnalignedRegion] if the region's start address is not 64-byte aligned.
    /// - [Error::UnalignedRegion] if the region's end address is not 63-byte aligned.
    /// - [Error::InvalidMair] if the region's MAIR index is invalid (greater than 7).
    pub fn set_region(&mut self, idx: u8, region: &El1Region) -> Result<(), Error> {
        let start = *(region.range.start()) as usize as u64;
        // Check for 64-byte alignment (0x3F is six bits)
        if start & 0x3F != 0 {
            return Err(Error::UnalignedRegion(region.range.clone()));
        }
        let end = *(region.range.end()) as usize as u64;
        if end & 0x3F != 0x3F {
            return Err(Error::UnalignedRegion(region.range.clone()));
        }
        if region.mair > 7 {
            return Err(Error::InvalidMair(region.mair));
        }
        registers::PRSELR_EL1.write(registers::PRSELR_EL1::Regions.val(idx as u64));
        registers::PRBAR_EL1.write(
            registers::PRBAR_EL1::Base.val(start >> 6)
                + registers::PRBAR_EL1::AP.val(region.access as u64)
                + registers::PRBAR_EL1::NX.val(if region.no_exec {
                    registers::PRBAR_EL1::NX::ExecutionNotPermitted.value
                } else {
                    registers::PRBAR_EL1::NX::ExecutionPermitted.value
                })
                + registers::PRBAR_EL1::SH.val(region.shareability as u64),
        );
        registers::PRLAR_EL1.write(
            registers::PRLAR_EL1::Limit.val(end >> 6)
                + registers::PRLAR_EL1::En.val(if region.enable {
                    registers::PRLAR_EL1::En::Enabled.value
                } else {
                    registers::PRLAR_EL1::En::Disabled.value
                })
                + registers::PRLAR_EL1::AttrIndex.val(region.mair as u64),
        );

        Ok(())
    }

    /// Writes a subset of EL1 MPU regions starting from a specified index.
    ///
    /// ## Arguments
    ///
    /// - `regions_starting_idx`: The starting index for the regions to be reconfigured.
    /// - `regions`: A slice of [El1Region] objects that will overwrite the previous regions
    ///   starting from `regions_starting_idx`.
    pub fn set_regions(
        &mut self,
        regions_starting_idx: u8,
        regions: &[El1Region],
    ) -> Result<(), Error> {
        let regions_used = regions.len().saturating_add(regions_starting_idx as usize);
        if regions_used > self.num_regions() as usize {
            return Err(Error::TooManyRegions(regions_used));
        }

        for (idx, region) in regions.iter().enumerate() {
            self.set_region(idx as u8 + regions_starting_idx, region)?;
        }

        Ok(())
    }

    /// Set up to eight memory attributes to MAIR_EL1.
    ///
    /// Eight attributes are stored within a single 64-bit register. If you pass fewer than eight
    /// memory attributes to this function, the remaining attributes will be set to all-zeroes. If
    /// you pass more than eight memory attributes to this function, anything after the first eight
    /// are ignored.
    #[allow(clippy::get_first)]
    pub fn set_attributes(&mut self, memattrs: &[MemAttr]) {
        let mem_attr0 = memattrs.get(0).map(|m| m.to_bits()).unwrap_or(0);
        let mem_attr1 = memattrs.get(1).map(|m| m.to_bits()).unwrap_or(0);
        let mem_attr2 = memattrs.get(2).map(|m| m.to_bits()).unwrap_or(0);
        let mem_attr3 = memattrs.get(3).map(|m| m.to_bits()).unwrap_or(0);
        let mem_attr4 = memattrs.get(4).map(|m| m.to_bits()).unwrap_or(0);
        let mem_attr5 = memattrs.get(5).map(|m| m.to_bits()).unwrap_or(0);
        let mem_attr6 = memattrs.get(6).map(|m| m.to_bits()).unwrap_or(0);
        let mem_attr7 = memattrs.get(7).map(|m| m.to_bits()).unwrap_or(0);
        let mair = u64::from_le_bytes([
            mem_attr0, mem_attr1, mem_attr2, mem_attr3, mem_attr4, mem_attr5, mem_attr6, mem_attr7,
        ]);
        registers::MAIR_EL1.set(mair);
        unsafe {
            core::arch::asm!("isb");
        }
    }

    /// Configure the EL1 MPU
    ///
    /// Write regions and attributes
    /// with a single [El1Config] struct.
    pub fn configure(&mut self, config: &El1Config) -> Result<(), Error> {
        self.set_regions(0, config.regions)?;

        self.set_attributes(config.memory_attributes);

        Ok(())
    }

    /// Enable the EL1 MPU
    pub fn enable(&mut self) {
        use registers::SCTLR_EL1::*;
        // Enable:
        //
        // * SCTLR_EL1.I (Instruction access Cacheability control)
        // * SCTLR_EL1.C (Data access Cacheability control)
        // * SCTLR_EL1.A (Alignment check)
        // * SCTLR_EL1.M (MPU enable for EL2 stage 1 and EL1&0 stage 2 address translation)
        unsafe {
            core::arch::asm!(r#"
                    dsb sy
                    isb
                    mrs {r:x}, SCTLR_EL1
                    orr {r:x}, {r:x}, {bits:x}
                    msr SCTLR_EL1, {r:x}
                    isb
                "#,
                r = inout(reg) 0 => _,
                bits = in(reg) I::SET.value + C::SET.value + A::SET.value + M::SET.value,
            );
        }
    }
}

/// Represents our PMSAv8-32 EL2 MPU
pub struct El2Mpu();

impl El2Mpu {
    /// Create an EL2 MPU handle
    ///
    /// # Safety
    ///
    /// Only create one of these at any given time, as they access shared
    /// mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> El2Mpu {
        El2Mpu()
    }

    /// How many MPU regions are there?
    pub fn num_regions(&self) -> u8 {
        registers::MPUIR_EL2.read(registers::MPUIR_EL2::Regions) as u8
    }

    /// Access the current state of a region
    pub fn get_region(&mut self, idx: u8) -> Option<El2Region> {
        if idx >= self.num_regions() {
            return None;
        }
        registers::PRSELR_EL2.write(registers::PRSELR_EL2::Regions.val(idx as u64));
        let prbar = registers::PRBAR_EL2.extract();
        let prlar = registers::PRLAR_EL2.extract();
        let start_addr = (prbar.read(registers::PRBAR_EL2::Base) << 6) as *const u8;
        let end_addr = ((prlar.read(registers::PRLAR_EL2::Limit) << 6) | 0x3F) as *const u8;
        let shareability = prbar.read_as_enum(registers::PRBAR_EL2::SH)?;
        let access = prbar.read_as_enum(registers::PRBAR_EL2::AP)?;
        let no_exec = prbar
            .read_as_enum::<registers::PRBAR_EL2::NX::Value>(registers::PRBAR_EL2::NX)?
            == registers::PRBAR_EL2::NX::Value::ExecutionNotPermitted;
        let mair = prlar.read(registers::PRLAR_EL2::AttrIndex) as u8;
        let enable = prlar
            .read_as_enum::<registers::PRLAR_EL2::En::Value>(registers::PRLAR_EL2::En)?
            == registers::PRLAR_EL2::En::Value::Enabled;
        Some(El2Region {
            range: start_addr..=end_addr,
            shareability,
            access,
            no_exec,
            mair,
            enable,
        })
    }

    /// Write a single region to the EL1 MPU
    ///
    /// ## Arguments
    ///
    /// - `region`: The [El1Region] object containing the configuration for the MPU region.
    /// - `idx`: The index of the region to be configured.
    ///
    /// ## Errors
    ///
    /// Returns:
    /// - [Error::UnalignedRegion] if the region's start address is not 64-byte aligned.
    /// - [Error::UnalignedRegion] if the region's end address is not 63-byte aligned.
    /// - [Error::InvalidMair] if the region's MAIR index is invalid (greater than 7).
    pub fn set_region(&mut self, idx: u8, region: &El2Region) -> Result<(), Error> {
        let start = *(region.range.start()) as usize as u64;
        // Check for 64-byte alignment (0x3F is six bits)
        if start & 0x3F != 0 {
            return Err(Error::UnalignedRegion(region.range.clone()));
        }
        let end = *(region.range.end()) as usize as u64;
        if end & 0x3F != 0x3F {
            return Err(Error::UnalignedRegion(region.range.clone()));
        }
        if region.mair > 7 {
            return Err(Error::InvalidMair(region.mair));
        }
        registers::PRSELR_EL2.write(registers::PRSELR_EL2::Regions.val(idx as u64));
        barrier::isb(barrier::SY);
        registers::PRBAR_EL2.write(
            registers::PRBAR_EL2::Base.val(start >> 6)
                + registers::PRBAR_EL2::AP.val(region.access as u64)
                + if region.no_exec {
                    registers::PRBAR_EL2::NX::ExecutionNotPermitted
                } else {
                    registers::PRBAR_EL2::NX::ExecutionPermitted
                }
                + registers::PRBAR_EL2::SH.val(region.shareability as u64),
        );
        registers::PRLAR_EL2.write(
            registers::PRLAR_EL2::Limit.val(end >> 6)
                + registers::PRLAR_EL2::En.val(if region.enable {
                    registers::PRLAR_EL2::En::Enabled.value
                } else {
                    registers::PRLAR_EL2::En::Disabled.value
                })
                + registers::PRLAR_EL2::AttrIndex.val(region.mair as u64),
        );
        barrier::isb(barrier::SY);
        Ok(())
    }

    /// Writes a subset of EL2 MPU regions starting from a specified index.
    ///
    /// ## Arguments
    ///
    /// - `regions_starting_idx`: The starting index for the regions to be reconfigured.
    /// - `regions`: A slice of [El2Region] objects that will overwrite the previous regions
    ///   starting from `regions_starting_idx`.
    pub fn set_regions(
        &mut self,
        regions_starting_idx: u8,
        regions: &[El2Region],
    ) -> Result<(), Error> {
        let regions_used = regions.len().saturating_add(regions_starting_idx as usize);
        if regions_used > self.num_regions() as usize {
            return Err(Error::TooManyRegions(regions_used));
        }

        for (idx, region) in regions.iter().enumerate() {
            self.set_region(idx as u8 + regions_starting_idx, region)?;
        }

        Ok(())
    }

    /// Set up to eight memory attributes to MAIR_EL2
    ///
    /// Eight attributes are stored within a single 64-bit register. If you pass fewer than eight
    /// memory attributes to this function, the remaining attributes will be set to all-zeroes. If
    /// you pass more than eight memory attributes to this function, anything after the first eight
    /// are ignored.
    #[allow(clippy::get_first)]
    pub fn set_attributes(&mut self, memattrs: &[MemAttr]) {
        let mem_attr0 = memattrs.get(0).map(|m| m.to_bits()).unwrap_or(0);
        let mem_attr1 = memattrs.get(1).map(|m| m.to_bits()).unwrap_or(0);
        let mem_attr2 = memattrs.get(2).map(|m| m.to_bits()).unwrap_or(0);
        let mem_attr3 = memattrs.get(3).map(|m| m.to_bits()).unwrap_or(0);
        let mem_attr4 = memattrs.get(4).map(|m| m.to_bits()).unwrap_or(0);
        let mem_attr5 = memattrs.get(5).map(|m| m.to_bits()).unwrap_or(0);
        let mem_attr6 = memattrs.get(6).map(|m| m.to_bits()).unwrap_or(0);
        let mem_attr7 = memattrs.get(7).map(|m| m.to_bits()).unwrap_or(0);
        let mair = u64::from_le_bytes([
            mem_attr0, mem_attr1, mem_attr2, mem_attr3, mem_attr4, mem_attr5, mem_attr6, mem_attr7,
        ]);
        registers::MAIR_EL2.set(mair);
        unsafe {
            core::arch::asm!("isb");
        }
    }

    /// Configure the EL2 MPU
    ///
    /// Write regions and attributes
    pub fn configure(&mut self, config: &El2Config) -> Result<(), Error> {
        self.set_regions(0, config.regions)?;

        self.set_attributes(config.memory_attributes);

        Ok(())
    }

    /// Enable the EL2 MPU
    pub fn enable(&mut self) {
        use registers::SCTLR_EL2::*;
        // Enable:
        //
        // * SCTLR_EL2.I (Instruction access Cacheability control)
        // * SCTLR_EL2.C (Data access Cacheability control)
        // * SCTLR_EL2.A (Alignment check)
        // * SCTLR_EL2.M (MPU enable for EL2 stage 1 and EL1&0 stage 2 address translation)
        unsafe {
            core::arch::asm!(r#"
                    dsb sy
                    isb
                    mrs {r:x}, SCTLR_EL2
                    orr {r:x}, {r:x}, {bits:x}
                    msr SCTLR_EL2, {r:x}
                    isb
                "#,
                r = inout(reg) 0 => _,
                bits = in(reg) I::SET.value + C::SET.value + A::SET.value + M::SET.value,
            );
        }
    }
}

/// Configuration for the PMSAv8-32 EL1 MPU
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct El1Config<'a> {
    /// Information about each Region.
    ///
    /// If you pass more regions than the MPU supports, you get an error.
    pub regions: &'a [El1Region],
    /// Information about each Memory Attribute
    ///
    /// If you pass more MemAttrs than the MPU supports (8), you get an error.
    pub memory_attributes: &'a [MemAttr],
}

/// Configuration for the PMSAv8-32 MPU
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct El1Region {
    /// The range of the region
    ///
    /// * The first address must be a multiple of 32.
    /// * The length must be a multiple of 32.
    pub range: core::ops::RangeInclusive<*const u8>,
    /// Shareability of the region
    pub shareability: El1Shareability,
    /// Access for the region
    pub access: El1AccessPerms,
    /// Is region no-exec?
    pub no_exec: bool,
    /// Which Memory Attribute applies here?
    ///
    /// Selects from the eight attributes in {MAIR0, MAIR1}.
    ///
    /// Only values 0..=7 are valid here.
    pub mair: u8,
    /// Is this region enabled?
    pub enable: bool,
}

// Creating a static Region is fine - the pointers within it
// only go to the MPU and aren't accessed via Rust code
unsafe impl Sync for El1Region {}

/// Configuration for the PMSAv8-32 EL2 MPU
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct El2Config<'a> {
    /// Information about each Region.
    ///
    /// If you pass more regions than the MPU supports, you get an error.
    pub regions: &'a [El2Region],
    /// Information about each Memory Attribute
    ///
    /// If you pass more MemAttrs than the MPU supports (8), you get an error.
    pub memory_attributes: &'a [MemAttr],
}

/// Configuration for the PMSAv8-32 EL2 MPU
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct El2Region {
    /// The range of the region
    ///
    /// * The first address must be a multiple of 32.
    /// * The length must be a multiple of 32.
    pub range: core::ops::RangeInclusive<*const u8>,
    /// Shareability of the region
    pub shareability: El2Shareability,
    /// Access for the region
    pub access: El2AccessPerms,
    /// Is region no-exec?
    pub no_exec: bool,
    /// Which Memory Attribute applies here?
    ///
    /// Selects from the eight attributes in {HMAIR0, HMAIR1}.
    ///
    /// Only values 0..=7 are valid here.
    pub mair: u8,
    /// Is this region enabled?
    pub enable: bool,
}

// Creating a static El2Region is fine - the pointers within it
// only go to the MPU and aren't accessed via Rust code
unsafe impl Sync for El2Region {}

/// Describes the memory ordering and cacheability of a region
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemAttr {
    /// Strongly-ordered memory (nGnRnE)
    StronglyOrdered,
    /// Early Write Ack (nGnRE)
    EarlyWriteAck,
    /// Reordering and Early Write Ack (nGRE)
    EarlyWriteAckReordering,
    /// Gathering, Reordering and Early Write Ack (GRE)
    GatheringEarlyWriteAckReordering,
    /// Normal memory
    NormalMemory {
        /// Controls outer access
        outer: Cacheable,
        /// Controls inner access
        inner: Cacheable,
    },
}

impl MemAttr {
    /// Convert this memory attribute to an 8-bit value we can write to MAIRx
    const fn to_bits(&self) -> u8 {
        match self {
            MemAttr::StronglyOrdered => 0b0000_0000,
            MemAttr::EarlyWriteAck => 0b0000_0100,
            MemAttr::EarlyWriteAckReordering => 0b0000_1000,
            MemAttr::GatheringEarlyWriteAckReordering => 0b0000_1100,
            MemAttr::NormalMemory { outer, inner } => {
                let outer_bits = outer.to_bits();
                let inner_bits = inner.to_bits();
                outer_bits << 4 | inner_bits
            }
        }
    }
}

/// Cacheability of a region
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cacheable {
    WriteThroughTransient(RwAllocPolicy),
    WriteBackTransient(RwAllocPolicy),
    WriteThroughNonTransient(RwAllocPolicy),
    WriteBackNonTransient(RwAllocPolicy),
    NonCacheable,
}

impl Cacheable {
    const fn to_bits(&self) -> u8 {
        #[allow(clippy::identity_op)]
        match self {
            Cacheable::WriteThroughTransient(rw_alloc) => 0b0000 | (*rw_alloc as u8),
            Cacheable::WriteBackTransient(rw_alloc) => 0b0100 | (*rw_alloc as u8),
            Cacheable::WriteThroughNonTransient(rw_alloc) => 0b1000 | (*rw_alloc as u8),
            Cacheable::WriteBackNonTransient(rw_alloc) => 0b1100 | (*rw_alloc as u8),
            Cacheable::NonCacheable => 0b0100,
        }
    }
}

/// Cache allocation policy
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum RwAllocPolicy {
    /// Write-allocate
    W = 0b01,
    /// Read-allocate
    R = 0b10,
    /// Read-allocate and Write-allocate
    RW = 0b11,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mem_attr_strong() {
        let mem_attr = MemAttr::StronglyOrdered;
        assert_eq!(mem_attr.to_bits(), 0b0000_0000);
    }

    #[test]
    fn mem_attr_device() {
        let mem_attr = MemAttr::EarlyWriteAck;
        assert_eq!(mem_attr.to_bits(), 0b0000_0100);
    }

    #[test]
    fn mem_attr_normal() {
        let mem_attr = MemAttr::NormalMemory {
            outer: Cacheable::NonCacheable,
            inner: Cacheable::WriteBackNonTransient(RwAllocPolicy::W),
        };
        assert_eq!(
            mem_attr.to_bits(),
            0b0100_1101,
            "0b{:08b}",
            mem_attr.to_bits()
        );
    }
}
