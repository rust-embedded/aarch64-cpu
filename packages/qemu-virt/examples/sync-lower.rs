//! Checks that taking an exception from EL1 to EL2 is handled by the right handler
// runner: qemu-system-aarch64 -cpu neoverse-v1 -machine virt,virtualization=on -nographic -semihosting -kernel

#![no_std]
#![no_main]

use core::arch::asm;

use aarch64_cpu::registers::{self, Readable as _};
use aarch64_pmsa_rt::{
    ExceptionHandlers, StackedRegisters, alloc_stack, drop_exception_level, entry,
    exception_handlers,
};
use semihosting::{println, process};

const EXPECTED_HVC_X0_ARG: u64 = 42;

entry!(main);

fn main() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    println!("running at EL{el}");
    assert_eq!(2, el, "this example must run at EL2");

    let stack = alloc_stack!(4096).expect("called twice");
    drop_exception_level(at_el1, stack);
}

extern "C" fn at_el1() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    println!("dropped to EL{el}");
    assert_eq!(1, el);

    // SAFETY: VBAR_EL2 has been set
    unsafe { asm!("HVC 0x00", in("x0") EXPECTED_HVC_X0_ARG) }

    process::exit(0)
}

exception_handlers!(Handlers);
struct Handlers;
impl ExceptionHandlers for Handlers {
    extern "C" fn sync_lower(context: &StackedRegisters) {
        let el = registers::CurrentEL.read(registers::CurrentEL::EL);

        println!("handling SVC at EL{el}");
        assert_eq!(2, el);
        assert_eq!(EXPECTED_HVC_X0_ARG, context.x[0]);
    }
}
