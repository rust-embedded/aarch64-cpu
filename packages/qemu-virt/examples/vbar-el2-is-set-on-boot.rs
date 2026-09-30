//! Checks that the RT library sets VBAR_EL2 when booting at EL2
// runner: qemu-system-aarch64 -cpu neoverse-v1 -machine virt,virtualization=on -nographic -semihosting -kernel

#![no_std]
#![no_main]

use core::{
    arch::asm,
    sync::atomic::{self, AtomicUsize},
};

use aarch64_cpu::registers::{self, Readable as _};
use aarch64_pmsa_rt::{ExceptionHandlers, StackedRegisters, entry, exception_handlers};
use semihosting::{eprintln, println, process};

entry!(main);

const EXPECTED_SVC_X0_ARG: u64 = 42;
const EXPECTED_HVC_X0_ARG: u64 = 24;

static COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    println!("running at EL{el}");
    assert_eq!(2, el, "this example must run at EL2");

    let vbar_el2 = registers::VBAR_EL2.get();
    eprintln!("VBAR_EL2={vbar_el2:#010x}");
    assert_ne!(0, vbar_el2, "VBAR_EL2 was not set");

    // SAFETY: VBAR_EL2 has been set
    unsafe { asm!("SVC 0x00", in("x0") EXPECTED_SVC_X0_ARG) }

    // SAFETY: VBAR_EL2 has been set
    unsafe { asm!("HVC 0x00", in("x0") EXPECTED_HVC_X0_ARG) }

    process::exit(0)
}

exception_handlers!(Handlers);
struct Handlers;
impl ExceptionHandlers for Handlers {
    extern "C" fn sync_current(context: &StackedRegisters) {
        let el = registers::CurrentEL.read(registers::CurrentEL::EL);
        match COUNT.fetch_add(1, atomic::Ordering::Relaxed) {
            0 => {
                println!("handling SVC at EL{el}");

                assert_eq!(EXPECTED_SVC_X0_ARG, context.x[0]);
            }
            1 => {
                println!("handling HVC at EL{el}");

                assert_eq!(EXPECTED_HVC_X0_ARG, context.x[0]);
            }
            _ => unreachable!(),
        }
    }
}
