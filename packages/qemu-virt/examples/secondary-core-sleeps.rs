//! Checks that secondary cores are put to sleep before entry point
// runner: qemu-system-aarch64 -cpu neoverse-v1 -machine virt,secure=on -nographic -semihosting -smp 2 -kernel

#![no_std]
#![no_main]

use aarch64_cpu::{
    asm,
    registers::{self, Readable},
};
use aarch64_pmsa_rt::{ExceptionHandlers, entry, exception_handlers};
use semihosting::{println, process};

entry!(main);

fn main() -> ! {
    // observably QEMU keeps all the cores but the first one halted when booting
    // at EL2 and lower so we need EL3 for this test
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    assert_eq!(3, el, "this example must run at EL3");

    // the QEMU invocation enables SMP with two cores
    // if the second core is sleeping then we should only see this once
    println!("MPIDR_EL1: {:#010x}", registers::MPIDR_EL1.get());

    // wait a bit because otherwise core that moves faster will
    // terminate the process before the other core gets a chance to print
    for _ in 0..10_000_000 {
        asm::nop();
    }

    process::exit(0)
}

exception_handlers!(NoOp);
struct NoOp;
impl ExceptionHandlers for NoOp {}
