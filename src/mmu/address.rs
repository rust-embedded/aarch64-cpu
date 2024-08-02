use super::descriptor::BlockDescriptor;
use core::{iter::StepBy, ops::Range};
use tock_registers::{fields::FieldValue, register_bitfields};

register_bitfields! {u64,
    VADescriptor [
        L0 OFFSET(39) NUMBITS(9) [],
        L1 OFFSET(30) NUMBITS(9) [],
        L2 OFFSET(21) NUMBITS(9) [],
        L3 OFFSET(12) NUMBITS(9) [],
        OFFSET OFFSET(0) NUMBITS(12) []
    ]
}

#[derive(Clone, Copy, Debug)]
pub enum PageMode {
    HugePage,
    LargePage,
    SmallPage,
}

impl PageMode {
    pub fn levels(&self) -> u8 {
        match self {
            PageMode::SmallPage => 3,
            PageMode::LargePage => 2,
            PageMode::HugePage => 1,
        }
    }
}

impl From<PageMode> for u64 {
    fn from(value: PageMode) -> Self {
        match value {
            PageMode::SmallPage => 0x1000,
            PageMode::LargePage => 0x200000,
            PageMode::HugePage => 0x40000000,
        }
    }
}

impl From<PageMode> for FieldValue<u64, BlockDescriptor::Register> {
    fn from(value: PageMode) -> Self {
        match value {
            PageMode::SmallPage => BlockDescriptor::TYPE::PAGE,
            _ => BlockDescriptor::TYPE::BLOCK,
        }
    }
}

#[derive(Copy, Clone)]
pub enum MMProt {
    NormalReadOnly,
    NormalExecOnly,
    NormalReadWriteAll,
    NormalReadWriteNoExec,
    PrivilegedExecOnly,
    PrivilegedReadOnly,
    PrivilegedReadWrite,
    VirtNone,
    VirtWriteOnly,
    VirtReadOnly,
    VirtReadOnlyNoExec,
    VirtReadWrite,
    VirtReadWriteNoExec,
    SecureExecOnly,
    SecureReadOnly,
    SecureReadWrite,
}

#[derive(Copy, Clone)]
pub enum MMType {
    Device,
    Normal,
    NormalNoExec,
    ReadOnly,
    Instruction,
    SystemReserved,
    SystemReadOnly,
    SystemInstruction,
    SecureReadOnly,
    SecureExecOnly,
    SecureReadWrite,
    VirtNone,
    VirtWriteOnly,
    VirtReadOnly,
    VirtReadOnlyNoExec,
    VirtReadWrite,
    VirtReadWriteNoExec,
}

pub struct VirtLayout {
    pub indexes: [usize; 4],
    pub offset: u64,
}

pub type MMSegment = (u64, u64);

#[derive(Clone, Copy, Debug)]
pub struct MMRegion {
    pub mem: MMSegment,
    pub granule: u64,
}

impl MMRegion {
    pub fn new(segment: MMSegment, granule: u64) -> Self {
        MMRegion {
            mem: (
                segment.0 & (!(granule - 1)),
                (segment.1 + granule - 1) & (!(granule - 1)),
            ),
            granule,
        }
    }
}

impl MMType {
    pub fn default_prot(&self) -> MMProt {
        match *self {
            Self::Normal => MMProt::NormalReadWriteAll,
            Self::NormalNoExec => MMProt::NormalReadWriteNoExec,
            Self::ReadOnly => MMProt::NormalReadOnly,
            Self::Instruction => MMProt::NormalExecOnly,
            Self::Device => MMProt::PrivilegedReadWrite,
            Self::SystemReserved => MMProt::PrivilegedReadWrite,
            Self::SystemInstruction => MMProt::PrivilegedExecOnly,
            Self::SystemReadOnly => MMProt::PrivilegedReadOnly,
            Self::VirtReadWrite => MMProt::VirtReadWrite,
            _ => unimplemented!(),
        }
    }
}

impl From<MMProt> for FieldValue<u64, BlockDescriptor::Register> {
    fn from(value: MMProt) -> Self {
        match value {
            MMProt::NormalReadWriteAll => {
                BlockDescriptor::UXN_XN::FALSE
                    + BlockDescriptor::PXN::FALSE
                    + BlockDescriptor::AP::RW_ELx_RW_EL0
                    + BlockDescriptor::NS::TRUE
            }
            MMProt::NormalReadWriteNoExec => {
                BlockDescriptor::UXN_XN::TRUE
                    + BlockDescriptor::PXN::TRUE
                    + BlockDescriptor::AP::RW_ELx_RW_EL0
                    + BlockDescriptor::NS::TRUE
            }
            MMProt::NormalExecOnly => {
                BlockDescriptor::UXN_XN::FALSE
                    + BlockDescriptor::PXN::FALSE
                    + BlockDescriptor::AP::RO_ELx_RO_EL0
                    + BlockDescriptor::NS::TRUE
            }
            MMProt::NormalReadOnly => {
                BlockDescriptor::UXN_XN::TRUE
                    + BlockDescriptor::PXN::TRUE
                    + BlockDescriptor::AP::RO_ELx_RO_EL0
                    + BlockDescriptor::NS::TRUE
            }
            MMProt::PrivilegedReadOnly => {
                BlockDescriptor::UXN_XN::TRUE
                    + BlockDescriptor::PXN::TRUE
                    + BlockDescriptor::AP::RO_ELx_None_EL0
                    + BlockDescriptor::NS::TRUE
            }
            MMProt::PrivilegedExecOnly => {
                BlockDescriptor::UXN_XN::TRUE
                    + BlockDescriptor::PXN::FALSE
                    + BlockDescriptor::AP::RO_ELx_None_EL0
                    + BlockDescriptor::NS::TRUE
            }
            MMProt::PrivilegedReadWrite => {
                BlockDescriptor::UXN_XN::TRUE
                    + BlockDescriptor::PXN::TRUE
                    + BlockDescriptor::AP::RW_ELx_None_EL0
                    + BlockDescriptor::NS::TRUE
            }
            MMProt::VirtReadWrite => {
                BlockDescriptor::UXN_XN::FALSE
                    + BlockDescriptor::PXN::FALSE
                    + BlockDescriptor::S2AP::WR
                    + BlockDescriptor::NS::TRUE
            }
            _ => unimplemented!(),
        }
    }
}

impl From<MMType> for FieldValue<u64, BlockDescriptor::Register> {
    fn from(value: MMType) -> Self {
        let prot_fields: FieldValue<u64, BlockDescriptor::Register> = value.default_prot().into();

        let type_fields = match value {
            MMType::Device => BlockDescriptor::SH::CLEAR,
            MMType::VirtReadWrite => BlockDescriptor::SH::OS,
            _ => BlockDescriptor::SH::IS + BlockDescriptor::NSE_NG::TRUE,
        };

        prot_fields + type_fields + BlockDescriptor::VALID::TRUE + BlockDescriptor::AF::TRUE
    }
}

#[derive(Copy, Clone)]
pub enum MMAttrIdx {
    Attr0 = 0,
    Attr1 = 1,
    Attr2 = 2,
    Attr3 = 3,
    Attr4 = 4,
    Attr5 = 5,
    Attr6 = 6,
    Attr7 = 7,
}

impl From<MMAttrIdx> for FieldValue<u64, BlockDescriptor::Register> {
    fn from(value: MMAttrIdx) -> Self {
        BlockDescriptor::ATTR.val(value as u64)
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum S2MemAttrNoFWB {
    Device_nGnRnE = 0b0000,
    Device_nGnRE = 0b0001,
    Device_nGRE = 0b0010,
    Device_GRE = 0b0011,
    // this is only for platform supported MTE_PERM,
    // if not, setting this will cause UNDEFINED BEHAVIOR.
    Normal_NTA_Outer_WBC_Inner_WBC_FEAT_MTE_PERM = 0b0100,

    Normal_Outer_NC_Inner_NC = 0b0101,
    Normal_Outer_NC_Inner_WTC = 0b0110,
    Normal_Outer_NC_Inner_WBC = 0b0111,

    Normal_Outer_WTC_Inner_NC = 0b1001,
    Normal_Outer_WTC_Inner_WTC = 0b1010,
    Normal_Outer_WTC_Inner_WBC = 0b1011,

    Normal_OuterWBC_Inner_NC = 0b1101,
    Normal_OuterWBC_Inner_WTC = 0b1110,
    Normal_OuterWBC_Inner_WBC = 0b1111,
}

impl From<S2MemAttrNoFWB> for FieldValue<u64, BlockDescriptor::Register> {
    fn from(value: S2MemAttrNoFWB) -> Self {
        BlockDescriptor::S2ATTR.val(value as u64)
    }
}

impl IntoIterator for MMRegion {
    type Item = u64;
    type IntoIter = StepBy<Range<u64>>;
    fn into_iter(self) -> Self::IntoIter {
        (self.mem.0..self.mem.1).step_by(self.granule as usize)
    }
}

impl From<u64> for VirtLayout {
    fn from(value: u64) -> Self {
        Self {
            indexes: [
                VADescriptor::L0.read(value) as usize,
                VADescriptor::L1.read(value) as usize,
                VADescriptor::L2.read(value) as usize,
                VADescriptor::L3.read(value) as usize,
            ],
            offset: VADescriptor::OFFSET.read(value),
        }
    }
}
