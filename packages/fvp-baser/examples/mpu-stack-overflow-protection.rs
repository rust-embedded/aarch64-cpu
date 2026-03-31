//! Checks that MPU can detect a stack overflow condition
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

    setup_rest_as_read_write();
    setup_stack_as_read_write();
    enable_mpu();
    println!("MPU has been enabled");

    let out = fibonacci(1_000);

    unreachable!("MPU fault was not raised (out={out})")
}

fn fibonacci(n: u64) -> u64 {
    // artifically use stack space
    let mut stack = [0u8; 32];
    let stackp = stack.as_mut_ptr() as usize;
    eprintln!("{:#010x}", stackp);

    if n < 2 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

const MAIR_IDX: u64 = 0;

fn setup_rest_as_read_write() {
    const REGION_IDX: u64 = 0;

    let (start, end) = (
        Section::Text.bottom() as usize,
        Section::Stack.bottom() as usize,
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

fn setup_stack_as_read_write() {
    let boot_stack = Section::Stack;
    let mid = boot_stack.bottom() as usize + 64;
    let regions = [
        (
            boot_stack.bottom() as usize,
            mid,
            registers::PRBAR_EL2::AP::ReadOnlyAtAnyEl,
            "RO",
        ),
        (
            mid,
            boot_stack.top() as usize,
            registers::PRBAR_EL2::AP::ReadWriteAtAnyEl,
            "RW",
        ),
    ];

    for ((start, end, ap, label), region_idx) in regions.into_iter().zip(1..) {
        eprintln!("{region_idx}. {start:#010x}..{end:#010x} {label}");

        registers::PRSELR_EL2.write(registers::PRSELR_EL2::Regions.val(region_idx));
        let region = mpu::address_range_to_protected_region(start..end);
        registers::PRBAR_EL2.write(
            registers::PRBAR_EL2::Base.val(region.base)
                + ap
                + registers::PRBAR_EL2::SH::NonShareable, // no SMP
        );
        registers::PRLAR_EL2.write(
            registers::PRLAR_EL2::Limit.val(region.limit)
                + registers::PRLAR_EL2::AttrIndex.val(MAIR_IDX)
                + registers::PRLAR_EL2::En::Enabled,
        );
    }
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
        // stack is exhausted so we cannot do much here
        process::exit(0)
    }
}
