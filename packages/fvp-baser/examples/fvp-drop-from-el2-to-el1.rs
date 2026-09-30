//! Checks that it's possible to drop from EL2 into EL1
// runner: FVP_BaseR_AEMv8R -f FVP_BaseR_AEMv8R.cfg

#![no_std]
#![no_main]

use core::arch::asm;

use aarch64_cpu::registers::{self, Readable};
use aarch64_pmsa_rt::{ExceptionHandlers, alloc_stack, entry, exception_handlers};
use semihosting::{eprintln, println, process};

entry!(main);

fn main() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    println!("running at EL{el}");
    assert_eq!(2, el, "this example must run at EL2");
    // the SP value is greatly affected by compiler optimizations (or the
    // lack of) so print it to stderr to omit it from the snapshot check
    eprintln!("(SP={:#010x})", read_stack_pointer());

    let stack = alloc_stack!(4096).expect("called twice");
    aarch64_pmsa_rt::drop_exception_level(at_el1, stack);
}

extern "C" fn at_el1() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    println!("dropped to EL{el}");
    assert_eq!(1, el);
    eprintln!("(SP={:#010x})", read_stack_pointer());

    process::exit(0)
}

#[inline(always)]
fn read_stack_pointer() -> usize {
    let value: usize;
    // SAFETY: always safe to read the Stack Pointer
    unsafe { asm!("mov {}, SP", out(reg) value) }
    value
}

exception_handlers!(NoOp);
struct NoOp;
impl ExceptionHandlers for NoOp {}
