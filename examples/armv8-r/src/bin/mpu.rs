//! Sets up MPU to protect all Normal and Device memory

#![no_std]
#![no_main]

use core::{
    fmt,
    sync::atomic::{self, AtomicU64},
};

use aarch64_cpu::{
    pmsav8,
    registers::{self, Readable as _},
};
use aarch64_pmsa_rt::{ExceptionHandlers, Section, entry, exception_handlers};
use armv8_r::Peripherals;
use semihosting::{eprintln, println, process};

entry!(main, stack_size = 65536, stack_align = 64);

static IN_BSS: AtomicU64 = AtomicU64::new(0);
static IN_DATA: AtomicU64 = AtomicU64::new(1);

fn main() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    println!("running at EL{el}");

    setup_mpu();

    // if we made it this far then `.stack`, `.text` and `.rodata` have been
    // correctly set up as we just used them in the preceding `println!`

    let mut mpu2 = unsafe { pmsav8::El2Mpu::new() };
    println!("Got {} EL2 MPU regions...", mpu2.num_regions());
    for i in 0..mpu2.num_regions() {
        println!("Region {:02}: {:010x?}", i, mpu2.get_region(i));
    }

    let mut mpu1 = unsafe { pmsav8::El1Mpu::new() };
    println!("Got {} EL1 MPU regions...", mpu1.num_regions());
    for i in 0..mpu1.num_regions() {
        println!("Region {:02}: {:010x?}", i, mpu1.get_region(i));
    }

    // test .bss
    let in_bss = IN_BSS.fetch_add(1, atomic::Ordering::Relaxed);
    println!(
        "IN_BSS={in_bss} (should be 0) @ {:p}",
        core::ptr::addr_of!(IN_BSS)
    );

    // test .data
    let in_data = IN_DATA.fetch_add(1, atomic::Ordering::Relaxed);
    println!(
        "IN_DATA={in_data} (should be 1) @ {:p}",
        core::ptr::addr_of!(IN_DATA)
    );

    // test device memory
    let Peripherals { mut gic, .. } = Peripherals::take();
    println!("Initialized peripherals, talking to GIC...");
    gic.set_interrupt_priority(arm_gic::IntId::sgi(0), 2);
    println!("All OK!");

    process::exit(0)
}

const MAIR_NORMAL_IDX: u8 = 0;
const MAIR_DEVICE_IDX: u8 = 1;

fn setup_mpu() {
    let mut mpu2 = unsafe { pmsav8::El2Mpu::new() };

    // RAM is Normal memory; its attributes are stored in MAIR entry 0
    // the attributes of Device memory are stored in MAIR entry 1
    mpu2.set_attributes(&[
        pmsav8::MemAttr::NormalMemory {
            outer: pmsav8::Cacheable::WriteThroughNonTransient(pmsav8::RwAllocPolicy::RW),
            inner: pmsav8::Cacheable::WriteThroughNonTransient(pmsav8::RwAllocPolicy::RW),
        },
        pmsav8::MemAttr::EarlyWriteAck,
    ]);

    let mut idx: u8 = 0;

    configure_normal_region(&mut mpu2, &mut idx, Section::Text, Perms::ReadExecute);
    configure_normal_region(&mut mpu2, &mut idx, Section::Rodata, Perms::ReadOnly);
    configure_normal_region(&mut mpu2, &mut idx, Section::Data, Perms::ReadWrite);
    configure_normal_region(&mut mpu2, &mut idx, Section::Bss, Perms::ReadWrite);
    configure_normal_region(&mut mpu2, &mut idx, Section::Uninit, Perms::ReadWrite);
    configure_normal_region(&mut mpu2, &mut idx, Section::Stack, Perms::ReadWrite);
    configure_device_region(&mut mpu2, &mut idx);

    mpu2.enable();
    println!("MPU has been enabled");
}

fn configure_normal_region(
    mpu: &mut pmsav8::El2Mpu,
    region_idx: &mut u8,
    region: Section,
    perms: Perms,
) {
    let Some(range) = region.mpu_range() else {
        return;
    };

    eprintln!(
        "{region_idx}. {:p}..{:p} Normal {perms}",
        *range.start(),
        *range.end()
    );

    mpu.set_region(
        *region_idx,
        &pmsav8::El2Region {
            range,
            shareability: pmsav8::El2Shareability::NonShareable,
            access: perms.access(),
            no_exec: perms.no_exec(),
            mair: MAIR_NORMAL_IDX,
            enable: true,
        },
    )
    .expect("Programming device region");

    *region_idx += 1;
}

fn configure_device_region(mpu: &mut pmsav8::El2Mpu, region_idx: &mut u8) {
    // from `arm-fvp-base-pac`
    const ETHERNET_START: *const u8 = 0x9A00_0000 as *const u8;
    const HDLCD_CONTROLLER_END: *const u8 = 0xFFF6_FFFF as *const u8;

    let range = ETHERNET_START..=HDLCD_CONTROLLER_END;

    eprintln!("{region_idx}. {:?} Device RW", range);

    mpu.set_region(
        *region_idx,
        &pmsav8::El2Region {
            range,
            shareability: pmsav8::El2Shareability::NonShareable,
            access: pmsav8::El2AccessPerms::ReadWriteAtAnyEl,
            no_exec: true,
            mair: MAIR_DEVICE_IDX,
            enable: true,
        },
    )
    .expect("Programming device region");

    *region_idx += 1;
}

#[allow(clippy::enum_variant_names)] // same prefix is fine because this is not exhaustive
#[derive(Clone, Copy)]
enum Perms {
    ReadOnly,
    ReadWrite,
    ReadExecute,
}

impl Perms {
    /// Get the MPU access permissions
    fn access(self) -> pmsav8::El2AccessPerms {
        match self {
            Perms::ReadOnly => pmsav8::El2AccessPerms::ReadOnlyAtAnyEl,
            Perms::ReadWrite => pmsav8::El2AccessPerms::ReadWriteAtAnyEl,
            Perms::ReadExecute => pmsav8::El2AccessPerms::ReadOnlyAtAnyEl,
        }
    }

    /// Should the NX bit be set, to prevent code execution
    fn no_exec(self) -> bool {
        match self {
            Perms::ReadOnly => true,
            Perms::ReadWrite => true,
            Perms::ReadExecute => false,
        }
    }
}

impl fmt::Display for Perms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Perms::ReadOnly => "RO",
            Perms::ReadWrite => "RW",
            Perms::ReadExecute => "RX",
        };
        f.write_str(s)
    }
}

// needed by aarch64-rt even when not used
exception_handlers!(Exceptions);
struct Exceptions;
impl ExceptionHandlers for Exceptions {}
