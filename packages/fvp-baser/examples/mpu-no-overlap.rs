//! Checks that protecting contiguous sections does not result in an overlap
//!
//! This mainly checks the logic in the `mpu` module
// runner: FVP_BaseR_AEMv8R -f FVP_BaseR_AEMv8R.cfg
// ignore: aarch64-unknown-none aarch64-unknown-none-softfloat
// features: v8-r

#![no_std]
#![no_main]

use core::sync::atomic::{self, AtomicU64};

use aarch64_cpu::registers::{self, ReadWriteable as _, Readable as _, Writeable as _};
use aarch64_pmsa_rt::{ExceptionHandlers, Section, entry, exception_handlers};
use semihosting::{eprintln, println, process};

mod mpu;

entry!(main);

static IN_DATA: AtomicU64 = AtomicU64::new(1);

fn main() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    println!("running at EL{el}");
    assert_eq!(2, el, "this example must run at EL2");

    // all sections/regions resides in RAM so we use one MAIR entry 0 for RAM
    registers::MAIR_EL2.write(
        registers::MAIR_EL2::Attr0_Normal_Inner::NonCacheable
            + registers::MAIR_EL2::Attr0_Normal_Outer::NonCacheable,
    );

    setup_text_rodata_as_read_only();
    setup_rest_as_read_write();
    enable_mpu();
    println!("MPU has been enabled");

    // if the protected regions overlap then this either raises a MPU fault or
    // results in "unpredictable" behavior
    let old = IN_DATA.fetch_add(1, atomic::Ordering::Relaxed);
    println!("IN_DATA={old}");
    assert_eq!(1, old);

    process::exit(0)
}

const MAIR_IDX: u64 = 0;

fn setup_text_rodata_as_read_only() {
    const REGION_IDX: u64 = 0;

    let (start, end) = (
        Section::Text.bottom() as usize,
        Section::Rodata.top() as usize,
    );
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

    assert_ne!(
        0,
        Section::Data.size(),
        "this test relies on .data not being empty"
    );
    assert_eq!(
        Section::Rodata.top() as usize,
        Section::Data.bottom() as usize,
        "this test relies on .rodata and .data being contiguous"
    );
    let (start, end) = (
        Section::Data.bottom() as usize,
        Section::Stack.top() as usize,
    );

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
impl ExceptionHandlers for Handlers {}
