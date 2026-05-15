//! Checks that secondary cores are put to sleep before entry point
// runner: FVP_BaseR_AEMv8R -f FVP_BaseR_AEMv8R_smp.cfg

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
    // sanity check; this FVP always starts at EL2
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    assert_eq!(2, el, "this example must run at EL2");

    // the FVP configuration starts two cores
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
