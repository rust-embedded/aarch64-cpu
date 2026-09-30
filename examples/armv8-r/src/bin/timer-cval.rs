//! Sets up a periodic timer interrupt using the CVAL register
//!
//! The CVAL holds a 64-bit compare value. When the timer count reaches this
//! compare value an interrupt is raised. Because the CVAL register is a
//! 64-bit value, larger interrupt periods can be achieved compared to TVAL.
//!
//! To avoid jitter, the previous CVAL value, instead of the current
//! count value which is how TVAL operates, can be used to compute the next
//! compare value.

#![no_std]
#![no_main]

use core::{
    cell::RefCell,
    sync::atomic::{self, AtomicUsize},
    time::Duration,
};

use aarch64_cpu::{
    asm,
    generic_timer::{El2PhysicalTimer, GenericTimer},
};
use aarch64_pmsa_rt::{ExceptionHandlers, StackedRegisters, entry, exception_handlers};
use armv8_r::gic::Gic;
use critical_section::Mutex;
use semihosting::println;

entry!(main);

const PERIOD: Duration = Duration::from_secs(1);

static SHARED: Mutex<RefCell<Option<El2PhysicalTimer>>> = Mutex::new(RefCell::new(None));

fn main() -> ! {
    println!("Hello, timer-cval starting up...");
    let armv8_r::Peripherals {
        mut gic,
        el2_phys_timer,
        ..
    } = armv8_r::Peripherals::take();

    println!("Configuring IRQ priority...");

    let intid = armv8_r::PHYS_INT_ID;
    gic.set_interrupt_priority(intid, 1);
    let enabled = true;
    gic.enable_interrupt(intid, enabled);

    arm_gic::irq_enable();
    gic.set_priority_mask(0);

    critical_section::with(|cs| {
        let mut borrow_mut = SHARED.borrow(cs).borrow_mut();
        let el2_phys_timer = borrow_mut.insert(el2_phys_timer);
        rearm_timer(el2_phys_timer, Some(el2_phys_timer.counter()), PERIOD);
    });

    println!("Sleeping...");

    loop {
        asm::wfi();
    }
}

exception_handlers!(Exceptions);
struct Exceptions;
impl ExceptionHandlers for Exceptions {
    extern "C" fn irq_current(_context: &StackedRegisters) {
        println!("IRQ!");
        while let Some(int_id) = Gic::get_and_acknowledge_interrupt() {
            if int_id == armv8_r::PHYS_INT_ID {
                handle_timer_irq();
            } else {
                unimplemented!()
            }
            Gic::end_interrupt(int_id);
        }
    }
}

fn handle_timer_irq() {
    static IRQ_COUNT: AtomicUsize = AtomicUsize::new(0);

    critical_section::with(|cs| {
        let mut borrow_mut = SHARED.borrow(cs).borrow_mut();
        let el2_phys_timer = borrow_mut.as_mut().unwrap();

        let start = el2_phys_timer.counter();
        el2_phys_timer.enable(false);
        rearm_timer(el2_phys_timer, None, PERIOD);

        let count = IRQ_COUNT.fetch_add(1, atomic::Ordering::Relaxed);
        println!("tick({count}) at {start}");
    });
}

fn rearm_timer(el2_phys_timer: &mut El2PhysicalTimer, baseline: Option<u64>, duration: Duration) {
    let msg = "duration is too large";
    let frequency_mhz = (el2_phys_timer.frequency_hz() / 1_000_000) as u64;
    let duration_us = u64::try_from(duration.as_micros()).expect(msg);
    let count = duration_us.checked_mul(frequency_mhz).expect(msg);
    let baseline = baseline.unwrap_or_else(|| el2_phys_timer.counter_compare());
    let next = baseline.wrapping_add(count);
    el2_phys_timer.counter_compare_set(next);
    el2_phys_timer.enable(true);
}
