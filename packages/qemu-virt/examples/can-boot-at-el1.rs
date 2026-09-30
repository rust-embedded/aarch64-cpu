//! Checks that the RT library supports booting at EL1
// runner: qemu-system-aarch64 -cpu neoverse-v1 -machine virt -nographic -semihosting -kernel

#![no_std]
#![no_main]

use aarch64_cpu::registers::{self, Readable};
use aarch64_pmsa_rt::{ExceptionHandlers, entry, exception_handlers};
use semihosting::{println, process};

entry!(main);

fn main() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    println!("running at EL{el}");
    assert_eq!(1, el, "this example must run at EL1");

    process::exit(0)
}

exception_handlers!(NoOp);
struct NoOp;
impl ExceptionHandlers for NoOp {}
