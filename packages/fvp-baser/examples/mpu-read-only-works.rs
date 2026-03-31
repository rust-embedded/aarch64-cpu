//! Checks that attempting to a read-only section raises a MPU fault
// runner: FVP_BaseR_AEMv8R -f FVP_BaseR_AEMv8R.cfg
// ignore: aarch64-unknown-none aarch64-unknown-none-softfloat
// features: v8-r

#![no_std]
#![no_main]

use core::sync::atomic;

use aarch64_cpu::registers::{self, ReadWriteable as _, Readable as _, Writeable as _};
use aarch64_pmsa_rt::{ExceptionHandlers, Section, StackedRegisters, entry, exception_handlers};
use semihosting::{eprintln, println, process};

mod mpu;

entry!(main);

fn main() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    println!("running at EL{el}");
    assert_eq!(2, el, "this example must run at EL2");

    // all sections/regions resides in RAM so we use one MAIR entry 0 for RAM
    registers::MAIR_EL2.write(
        registers::MAIR_EL2::Attr0_Normal_Inner::NonCacheable
            + registers::MAIR_EL2::Attr0_Normal_Outer::NonCacheable,
    );

    setup_text_as_read_only();
    setup_rest_as_read_write();
    enable_mpu();
    println!("MPU has been enabled");

    unsafe extern "C" {
        static mut _start: u32;
    }

    println!("attempting to overwrite entry point");
    // try to overwrite the entry point
    // SAFETY: intentional unsound operation that will be runtime checked
    unsafe {
        let startp = &raw mut _start;
        eprintln!("entry point is at {:#010x}", startp as usize);
        let before = startp.read_volatile();
        println!("contents before write: {before:#010x}");
        startp.write_volatile(0x0);
        let after = startp.read_volatile();
        println!("contents after write: {after:#010x}");
    }

    unreachable!("MPU fault was not raised")
}

const MAIR_IDX: u64 = 0;

fn setup_text_as_read_only() {
    const REGION_IDX: u64 = 0;

    let text = Section::Text;
    let (start, end) = (text.bottom() as usize, text.top() as usize);
    eprintln!("{REGION_IDX}. {start:#010x}..{end:#010x} RO");

    registers::PRSELR_EL2.write(registers::PRSELR_EL2::Regions.val(REGION_IDX));
    let region = mpu::address_range_to_protected_region(start..end);
    registers::PRBAR_EL2.write(
        registers::PRBAR_EL2::Base.val(region.base)
            + registers::PRBAR_EL2::AP::ReadOnlyAtAnyEl
            + registers::PRBAR_EL2::SH::NonShareable, // no SMP
    );
    registers::PRLAR_EL2.write(
        registers::PRLAR_EL2::Limit.val(region.limit)
            + registers::PRLAR_EL2::AttrIndex.val(MAIR_IDX)
            + registers::PRLAR_EL2::En::Enabled,
    );
}

fn setup_rest_as_read_write() {
    const REGION_IDX: u64 = 1;

    let (start, end) = (Section::Text.top() as usize, Section::Stack.top() as usize);

    eprintln!("{REGION_IDX}. {start:#010x}..{end:#010x} RW");
    registers::PRSELR_EL2.write(registers::PRSELR_EL2::Regions.val(REGION_IDX));
    let region = mpu::address_range_to_protected_region(start..end);
    registers::PRBAR_EL2.write(
        registers::PRBAR_EL2::Base.val(region.base)
            + registers::PRBAR_EL2::AP::ReadWriteAtAnyEl
            + registers::PRBAR_EL2::SH::NonShareable, // no SMP
    );
    registers::PRLAR_EL2.write(
        registers::PRLAR_EL2::Limit.val(region.limit)
            + registers::PRLAR_EL2::AttrIndex.val(MAIR_IDX)
            + registers::PRLAR_EL2::En::Enabled,
    );
}

fn enable_mpu() {
    registers::SCTLR_EL2.modify(registers::SCTLR_EL2::M::Enable);
    // prevent memory operations after the preceding write from being
    // moved/re-ordered before the write
    atomic::fence(atomic::Ordering::Release);
}

exception_handlers!(Handlers);
struct Handlers;
impl ExceptionHandlers for Handlers {
    extern "C" fn sync_current(_context: &StackedRegisters) {
        // [ARM-R64/D12.2.31] Exception Class for Data Abort, including
        // MPU faults
        const EC_DATA_ABORT_MPU_FAULT: u64 = 0b100101;
        // [ARM-R64/D12.2.31] Data Fault Status Code for permission fault
        const DFSC_PERM_FAULT_LEVEL_0: u64 = 0b001100;

        let ec = registers::ESR_EL2.read(registers::ESR_EL2::EC);
        assert_eq!(
            EC_DATA_ABORT_MPU_FAULT, ec,
            "Exception Class does not indicate an MPU fault"
        );
        let iss = registers::ESR_EL2.read(registers::ESR_EL2::ISS);
        let dfsc = iss & (0b11_1111);
        assert_eq!(
            DFSC_PERM_FAULT_LEVEL_0, dfsc,
            "Data Fault Status Code does not indicate a permission issue"
        );

        println!("MPU fault raised");

        process::exit(0)
    }
}
