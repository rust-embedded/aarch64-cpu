//! Sets up a periodic timer interrupt using the TVAL register
//!
//! The TVAL register holds a signed 32-bit value that represents a
//! duration until the next interrupt. The fact that is a 32-bit value limits
//! how large the interrupt period can be. Also, the fact that TVAL is a
//! "Duration" and not an "Instant" makes it easy to introduce jitter.

#![no_std]
#![no_main]

use core::{cell::RefCell, time::Duration};

use aarch64_cpu::{
    asm,
    generic_timer::{El2PhysicalTimer, GenericTimer},
};
use aarch64_pmsa_rt::{ExceptionHandlers, StackedRegisters, entry, exception_handlers};
use armv8_r::gic::Gic;
use critical_section::Mutex;
use semihosting::println;

entry!(main);

const SCALE: u32 = 1 << 6;
const PERIOD: Duration = Duration::from_secs(1);

fn scaled_period() -> Duration {
    PERIOD / SCALE
}

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
        arm_timer(el2_phys_timer, scaled_period());
    });

    println!("Sleeping...");

    loop {
        asm::wfi()
    }
}

exception_handlers!(Exceptions);
struct Exceptions;
impl ExceptionHandlers for Exceptions {
    extern "C" fn irq_current(_context: &StackedRegisters) {
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
    use core::sync::atomic::{self, AtomicUsize};

    static IRQ_COUNT: AtomicUsize = AtomicUsize::new(0);

    let start = critical_section::with(|cs| {
        let mut borrow_mut = SHARED.borrow(cs).borrow_mut();
        let el2_phys_timer = borrow_mut.as_mut().unwrap();
        let start = el2_phys_timer.counter();
        el2_phys_timer.enable(false);
        arm_timer(el2_phys_timer, scaled_period());
        start
    });

    let count = IRQ_COUNT.fetch_add(1, atomic::Ordering::Relaxed);
    if count.is_multiple_of(SCALE as usize) {
        println!("tick({}) at {start}", count / SCALE as usize);
    }
}

fn arm_timer(el2_phys_timer: &mut El2PhysicalTimer, duration: Duration) {
    let msg = "duration is too large";
    let frequency_mhz = (el2_phys_timer.frequency_hz() / 1_000_000) as u32;
    let duration_us = u32::try_from(duration.as_micros()).expect(msg);
    let count = duration_us.checked_mul(frequency_mhz).expect(msg);
    el2_phys_timer.countdown_set(count);
    el2_phys_timer.enable(true);
}
