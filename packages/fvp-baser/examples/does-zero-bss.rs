//! Checks that the startup code does zero the `.bss` section
// runner: FVP_BaseR_AEMv8R -f FVP_BaseR_AEMv8R.cfg
// the FVP fills unimplemented and volatile memory with a non-zero byte pattern
// whereas QEMU fills it with zeroes so we can only run this test on the FVP

#![no_std]
#![no_main]

use core::sync::atomic::AtomicU32;

use aarch64_pmsa_rt::{ExceptionHandlers, entry, exception_handlers};
use semihosting::{println, process};

static IN_BSS: AtomicU32 = AtomicU32::new(0);

entry!(main);

fn main() -> ! {
    // use volatile to force a load instruction and prevent the compiler from
    // giving us a cached const value
    // SAFETY: not different from load(Relaxed)
    let initial_value = unsafe { IN_BSS.as_ptr().read_volatile() };
    println!("static variable's initial value: {initial_value:#010x}");
    assert_eq!(0, initial_value);

    process::exit(0)
}

exception_handlers!(NoOp);
struct NoOp;
impl ExceptionHandlers for NoOp {}
