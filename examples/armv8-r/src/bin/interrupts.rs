//! Example of nested Software Generated Interrupts (SGI)

#![no_main]
#![no_std]

use aarch64_pmsa_rt::{ExceptionHandlers, StackedRegisters, entry, exception_handlers};
use semihosting::{println, process};

use armv8_r::{
    Peripherals,
    fmt::Nesting,
    gic::{Gic, IntId},
};

// default stack size (4 KiB) is too little and results in a stack overflow
entry!(main, stack_size = 8 * 1024);

const INTID_SGI0: IntId = IntId::sgi(0);
const INTID_SGI1: IntId = IntId::sgi(1);

fn main() -> ! {
    println!("start of main");

    let Peripherals { mut gic, .. } = Peripherals::take();

    let interrupts_and_priorities = [(INTID_SGI0, 2), (INTID_SGI1, 1)];
    for (intid, logical_prio) in interrupts_and_priorities {
        gic.set_interrupt_priority(intid, logical_prio);
    }

    let enabled = true;
    for (intid, _logical_prio) in interrupts_and_priorities {
        gic.enable_interrupt(intid, enabled);
    }

    // needed because ISA exceptions handlers (e.g. IRQ) are masked out of boot
    arm_gic::irq_enable();

    // on boot this is set to the highest logical priority which masks all interrupts, so
    // it needs to be loosened
    gic.set_priority_mask(0);

    Gic::send_sgi(INTID_SGI1);

    println!("end of main");

    process::exit(0)
}

exception_handlers!(Exceptions);
struct Exceptions;
impl ExceptionHandlers for Exceptions {
    extern "C" fn irq_current(_context: &StackedRegisters) {
        let _guard = Nesting::increase();
        println!("{Nesting}> irq_current()");

        while let Some(int_id) = Gic::get_and_acknowledge_interrupt() {
            println!("{Nesting}Handling {:?}", int_id);

            arm_gic::irq_enable();
            if int_id == INTID_SGI0 {
                handle_sgi0_irq();
            } else if int_id == INTID_SGI1 {
                handle_sgi1_irq();
            } else {
                unimplemented!()
            }
            arm_gic::irq_disable();

            println!("{Nesting}Handled {:?}", int_id);
            Gic::end_interrupt(int_id);
            println!("{Nesting}< irq_current()");
        }
    }
}

fn handle_sgi0_irq() {
    println!("{Nesting}SGI0 handler")
}

fn handle_sgi1_irq() {
    println!("{Nesting}SGI1 handler");

    println!("{Nesting}before raising SGI0");
    Gic::send_sgi(INTID_SGI0);
    println!("{Nesting}after raising SGI0");
}
