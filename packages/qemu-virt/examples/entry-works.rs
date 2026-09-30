//! Checks that the entry point is executed
// runner: qemu-system-aarch64 -cpu neoverse-v1 -machine virt -nographic -semihosting -kernel

#![no_std]
#![no_main]

use aarch64_pmsa_rt::{ExceptionHandlers, entry, exception_handlers};
use semihosting::{println, process};

entry!(main);

fn main() -> ! {
    println!("main was called");

    process::exit(0)
}

exception_handlers!(NoOp);
struct NoOp;
impl ExceptionHandlers for NoOp {}
