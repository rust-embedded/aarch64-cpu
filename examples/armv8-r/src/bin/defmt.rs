#![no_main]
#![no_std]

use aarch64_rt::{ExceptionHandlers, RegisterStateRef, entry, exception_handlers};
use semihosting::{println, process};

use armv8_r::{
    Peripherals,
    fmt::Nesting,
    gic::{Gic, IntId},
};

entry!(main);

const INTID_SGI0: IntId = IntId::sgi(0);

fn main(_arg0: u64, _arg1: u64, _arg2: u64, _arg3: u64) -> ! {
    let Peripherals { mut gic } = Peripherals::take();

    gic.set_interrupt_priority(INTID_SGI0, 1);

    let enabled = true;
    gic.enable_interrupt(INTID_SGI0, enabled);

    arm_gic::irq_enable();

    gic.set_priority_mask(0);

    println!("before defmt log");
    defmt::info!(">>> {} <<<", SendSgi(42));
    println!("after defmt log");

    process::exit(0)
}

struct SendSgi(i32);

impl defmt::Format for SendSgi {
    fn format(&self, f: defmt::Formatter<'_>) {
        println!("before raise SGI0");
        // this interrupt should be withheld until the defmt logger is released
        Gic::send_sgi(INTID_SGI0);
        println!("after raise SGI0");
        self.0.format(f);
    }
}

exception_handlers!(Exceptions);
struct Exceptions;
impl ExceptionHandlers for Exceptions {
    extern "C" fn irq_current(_register_state: RegisterStateRef<'_>) {
        let _guard = Nesting::increase();
        println!("{Nesting}> irq_current()");
        while let Some(int_id) = Gic::get_and_acknowledge_interrupt() {
            println!("{Nesting}Handling {:?}", int_id);
            arm_gic::irq_enable();
            if int_id == INTID_SGI0 {
                handle_sgi0_irq();
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
    defmt::info!("SGI0 handler")
}
