//! Checks that nesting exceptions taken to the same EL works
// runner: qemu-system-aarch64 -cpu neoverse-v1 -machine virt -nographic -semihosting -kernel

#![no_std]
#![no_main]

use core::{
    arch::asm,
    sync::atomic::{self, AtomicUsize},
};

use aarch64_cpu::registers::{self, Readable as _};
use aarch64_pmsa_rt::{ExceptionHandlers, StackedRegisters, entry, exception_handlers};
use semihosting::{println, process};

entry!(main);

fn main() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    println!("running at EL{el}");
    assert_eq!(1, el, "this example must run at EL1");

    // SAFETY: VBAR_EL1 has been set
    unsafe { asm!("SVC 0x00") }

    process::exit(0)
}

static COUNT: AtomicUsize = AtomicUsize::new(0);

exception_handlers!(Handlers);
struct Handlers;
impl ExceptionHandlers for Handlers {
    extern "C" fn sync_current(context: &StackedRegisters) {
        let count = COUNT.fetch_add(1, atomic::Ordering::Relaxed);

        match count {
            0 => {
                println!("handling outer Sync exception");
                let elr_before = registers::ELR_EL1.get() as usize;

                assert_eq!(context.elr, elr_before);

                // SAFETY: VBAR_EL1 has been set and exception handler is
                // re-entrant safe
                unsafe { asm!("SVC 0x00") }
                let elr_after = registers::ELR_EL1.get() as usize;

                // handling the second SVC overwrites ELR_EL1 but if the
                // epilogue works then we'll still return to the right place
                assert_ne!(elr_before, elr_after);
            }
            1 => {
                println!("handling nested Sync exception");
            }
            _ => unreachable!(),
        }
    }
}
