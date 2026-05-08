//! A defmt logger over UART0

use core::{
    cell::UnsafeCell,
    ptr::NonNull,
    sync::atomic::{self, AtomicBool},
};

use arm_fvp_base_pac::{UniqueMmioPointer, arm_pl011_uart::Uart};
use critical_section::RestoreState;

#[defmt::global_logger]
struct Logger;

static DEFMT_UART: DefmtUart = DefmtUart::new();

struct DefmtUart {
    encoder: UnsafeCell<defmt::Encoder>,
    taken: AtomicBool,
    cs_restore: UnsafeCell<RestoreState>,
}

impl DefmtUart {
    const fn new() -> Self {
        Self {
            encoder: UnsafeCell::new(defmt::Encoder::new()),
            taken: AtomicBool::new(false),
            cs_restore: UnsafeCell::new(RestoreState::invalid()),
        }
    }

    fn acquire(&self) {
        let restore = unsafe { critical_section::acquire() };

        if self.taken.load(atomic::Ordering::Relaxed) {
            panic!("defmt logger taken reentrantly")
        }

        self.taken.store(true, atomic::Ordering::Relaxed);

        // SAFETY: inside a critical section
        unsafe {
            self.cs_restore.get().write(restore);
            (*self.encoder.get()).start_frame(|encoded| {
                for byte in encoded {
                    write_byte(*byte)
                }
            })
        }
    }

    unsafe fn flush(&self) {
        // no action
    }

    unsafe fn release(&self) {
        if !self.taken.load(atomic::Ordering::Relaxed) {
            // release out of context
            return;
        }

        // SAFETY: inside a critical section
        unsafe {
            (*self.encoder.get()).end_frame(|encoded| {
                for byte in encoded {
                    write_byte(*byte)
                }
            })
        }

        // this flag needs to be cleared before interrupts are allowd to rerun or they may
        // hit the 'taken reentrantly' panic branch in `acquire`
        self.taken.store(false, atomic::Ordering::Relaxed);
        // end of critical section
        unsafe {
            let restore = self.cs_restore.get().read();
            critical_section::release(restore);
        }
    }

    unsafe fn write(&self, bytes: &[u8]) {
        if !self.taken.load(atomic::Ordering::Relaxed) {
            // write out of critical section
            return;
        }

        // SAFETY: inside a critical section
        unsafe {
            (*self.encoder.get()).write(bytes, |encoded| {
                for byte in encoded {
                    write_byte(*byte)
                }
            })
        }
    }
}

unsafe impl defmt::Logger for Logger {
    fn acquire() {
        DEFMT_UART.acquire()
    }

    unsafe fn flush() {
        unsafe { DEFMT_UART.flush() }
    }

    unsafe fn release() {
        unsafe { DEFMT_UART.release() }
    }

    unsafe fn write(bytes: &[u8]) {
        unsafe { DEFMT_UART.write(bytes) }
    }
}

unsafe impl Sync for DefmtUart {}

fn write_byte(byte: u8) {
    // SAFETY: this UART instance is only used from within this critical section
    let uart0 = unsafe { arm_fvp_base_pac::Peripherals::steal().uart0 };
    let mut uart =
        Uart::new(unsafe { UniqueMmioPointer::new(NonNull::new(uart0.pa() as *mut _).unwrap()) });

    uart.write_word(byte);
}
