//! Checks that it's possible to drop from EL2 into EL1
// runner: qemu-system-aarch64 -cpu neoverse-v1 -machine virt,virtualization=on -nographic -semihosting -kernel

#![no_std]
#![no_main]

use core::{
    arch::asm,
    sync::atomic::{self, AtomicUsize},
};

use aarch64_cpu::registers::{self, Readable};
use aarch64_pmsa_rt::{ExceptionHandlers, alloc_stack, entry, exception_handlers};
use semihosting::{eprintln, println, process};

entry!(main);

const SP_MASK: usize = !((1 << 12) - 1);
static SPECIFIED_SP: AtomicUsize = AtomicUsize::new(0);

fn main() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    let sp = read_stack_pointer();
    println!("running at EL{el}");
    assert_eq!(2, el, "this example must run at EL2");
    // the SP value is greatly affected by compiler optimizations (or the lack
    // of) so print it unmasked to stderr to omit it from the snapshot check
    eprintln!("(SP={sp:#010x})");

    // make the alignment match the size so, that after masking SP, we can check
    // that the lower EL is using the SP we configured here
    let stack = alloc_stack!(4_096, align = 4_096).expect("called twice");
    SPECIFIED_SP.store(stack.lower(), atomic::Ordering::Relaxed);
    aarch64_pmsa_rt::drop_exception_level(at_el1, stack);
}

extern "C" fn at_el1() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    let sp = read_stack_pointer();
    println!("dropped to EL{el}");
    let actual_sp = sp & SP_MASK;
    assert_eq!(1, el);
    eprintln!("(SP={sp:#010x})");

    let specified_sp = SPECIFIED_SP.load(atomic::Ordering::Relaxed);
    assert_eq!(
        specified_sp, actual_sp,
        "EL1 is not using the configured Stack Pointer"
    );

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
