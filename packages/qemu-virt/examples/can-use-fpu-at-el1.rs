//! Checks that the RT library enables use of FPU at EL1
// runner: qemu-system-aarch64 -cpu neoverse-v1 -machine virt -nographic -semihosting -kernel
// ignore: aarch64-unknown-none-softfloat

#![no_std]
#![no_main]

use core::arch::asm;

use aarch64_cpu::registers::{self, Readable};
use aarch64_pmsa_rt::{ExceptionHandlers, entry, exception_handlers};
use semihosting::{println, process};

entry!(main);

fn main() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    println!("running at EL{el}");
    assert_eq!(1, el, "this example must run at EL1");

    println!("FPU result: {}", add(1.0, 2.0));

    process::exit(0)
}

fn add(x: f64, y: f64) -> f64 {
    let out: f64;
    // use assembly to ensure a FPU instruction is emitted; the compiler can
    // const-compute the result when constant inputs are provided
    // SAFETY: "pure" instructions without side effects
    unsafe {
        asm!(
            "fadd {:d}, {:d}, {:d}",
            out(vreg) out, in(vreg) x, in(vreg) y,
            options(nomem, pure),
        )
    }
    out
}

exception_handlers!(NoOp);
struct NoOp;
impl ExceptionHandlers for NoOp {}
