#![no_std]
#![no_main]

use core::sync::atomic::{self, AtomicU64};

use aarch64_cpu::registers::{self, Readable};
use aarch64_pmsa_rt::{ExceptionHandlers, entry, exception_handlers};
use semihosting::{println, process};

static X: AtomicU64 = AtomicU64::new(0);
static Y: AtomicU64 = AtomicU64::new(1);

entry!(main);

fn main() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    println!("Hello, world! running from EL{el}");

    let x = X.fetch_add(1, atomic::Ordering::Relaxed);
    let y = Y.fetch_add(1, atomic::Ordering::Relaxed);
    println!("static variables: X={x}, Y={y}");

    // sanity check that static variables are initialized by `aarch64-rt`
    assert_eq!(0, x, ".bss was not initialized");
    assert_eq!(1, y, ".data was not initialized");

    process::exit(0)
}

// needed by aarch64-rt even when not used
exception_handlers!(Exceptions);
struct Exceptions;
impl ExceptionHandlers for Exceptions {}
