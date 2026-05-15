//! Checks that the RT library sets VBAR_EL1 when booting at EL1
// runner: qemu-system-aarch64 -cpu neoverse-v1 -machine virt -nographic -semihosting -kernel

#![no_std]
#![no_main]

use core::arch::asm;

use aarch64_cpu::registers::{self, Readable as _};
use aarch64_pmsa_rt::{ExceptionHandlers, StackedRegisters, entry, exception_handlers};
use semihosting::{eprintln, println, process};

entry!(main);

const EXPECTED_SVC_X0_ARG: u64 = 42;

fn main() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    println!("running at EL{el}");
    assert_eq!(1, el, "this example must run at EL1");

    let vbar_el1 = registers::VBAR_EL1.get();
    eprintln!("VBAR_EL1={vbar_el1:#010x}");
    assert_ne!(0, vbar_el1, "VBAR_EL1 was not set");

    // SAFETY: VBAR_EL1 has been set
    unsafe { asm!("SVC 0x00", in("x0") EXPECTED_SVC_X0_ARG) }

    process::exit(0)
}

exception_handlers!(Handlers);
struct Handlers;
impl ExceptionHandlers for Handlers {
    extern "C" fn sync_current(context: &StackedRegisters) {
        let el = registers::CurrentEL.read(registers::CurrentEL::EL);
        println!("handling SVC at EL{el}");

        assert_eq!(EXPECTED_SVC_X0_ARG, context.x[0]);
    }
}
