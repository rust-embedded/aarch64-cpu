//! Support for Arm Generic Timer
//!
//! See Chapter D12 The Generic Timer in AArch64 state [ARM Architecture
//! Reference Manual v8][armv8].
//!
//! [armv8]: https://developer.arm.com/documentation/ddi0487/latest/

mod el2;
pub use el2::{El2HypPhysicalTimer, El2PhysicalTimer, El2VirtualTimer};

mod el1;
pub use el1::{El1PhysicalTimer, El1VirtualTimer};

mod el0;
pub use el0::{El0PhysicalTimer, El0VirtualTimer};

/// Describes either a Physical or Virtual timer
pub trait GenericTimer {
    /// Get the timer frequency
    fn frequency_hz(&self) -> u32;

    /// Get the current counter value.
    ///
    /// This is a 64-bit value that goes up. It can be used to measure the
    /// passing of time.
    ///
    /// Note that speculative reads are allowed and reads may occur out of
    /// order. Add ISB and DSB fences before/after this call as required.
    fn counter(&self) -> u64;

    /// Get the counter compare value.
    fn counter_compare(&self) -> u64;

    /// Set the counter compare value.
    ///
    /// The timer condition is met when the `counter - compare_value >= 0`. This
    /// therefore operates as a *count-up* timer.
    fn counter_compare_set(&mut self, value: u64);

    /// Get the current value of the countdown timer.
    fn countdown(&self) -> u32;

    /// Set the value of the count-down timer.
    ///
    /// Sets a value which is decremented on every counter tick. When it reaches
    /// zero, the timer fires.
    fn countdown_set(&mut self, duration_ticks: u32);

    /// This is timer enabled?
    fn enabled(&self) -> bool;

    /// Enable/disable this timer
    fn enable(&self, enabled: bool);

    /// Is this timer's interrupt masked?
    fn interrupt_masked(&self) -> bool;

    /// Mask (or unmask) this timer's interrupt.
    fn interrupt_mask(&mut self, mask: bool);

    /// Has this timer's interrupt fired?
    fn interrupt_status(&self) -> bool;

    /// Wait for some number of clock ticks
    fn delay_ticks(&mut self, ticks: u32) {
        let enabled = self.enabled();
        self.enable(true);
        self.countdown_set(ticks);
        while !self.interrupt_status() {
            core::hint::spin_loop();
        }
        if !enabled {
            self.enable(false);
        }
    }

    /// Delay for some number of milliseconds
    fn delay_ms(&mut self, ms: u32) {
        // can't overflow a u64 if you multiply two u32s together
        let mut ticks: u64 = u64::from(self.frequency_hz()).wrapping_mul(u64::from(ms)) / 1000;
        while ticks >= 0xFFFF_FFFF {
            self.delay_ticks(0xFFFF_FFFF);
            ticks -= 0xFFFF_FFFF;
        }
        self.delay_ticks(ticks as u32);
    }

    /// Delay for some number of microseconds
    fn delay_us(&mut self, us: u32) {
        // can't overflow a u64 if you multiply two u32s together
        let mut ticks: u64 = u64::from(self.frequency_hz()).wrapping_mul(u64::from(us)) / 1_000_000;
        while ticks >= 0xFFFF_FFFF {
            self.delay_ticks(0xFFFF_FFFF);
            ticks -= 0xFFFF_FFFF;
        }
        self.delay_ticks(ticks as u32);
    }
}

/// Describes the configuration for Timer Events
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventConfig {
    /// Controls which transition of the CNTVCT trigger bit, defined by EVNTI,
    /// generates an event, when the event stream is enabled.
    pub evntdir: EventDir,
    /// How often does the event fire
    pub rate: EventRate,
}

/// Describes the direction of an Event
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EventDir {
    /// Event fires on a 1 -> 0 transition
    HighLow,
    /// Event fires on a 0 -> 1 transition
    LowHigh,
}

/// How often does the event fire?
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum EventRate {
    /// Fire every 1 ticks
    _1 = 0,
    /// Fire every 2 ticks
    _2 = 1,
    /// Fire every 4 ticks
    _4 = 2,
    /// Fire every 8 ticks
    _8 = 3,
    /// Fire every 16 ticks
    _16 = 4,
    /// Fire every 32 ticks
    _32 = 5,
    /// Fire every 64 ticks
    _64 = 6,
    /// Fire every 128 ticks
    _128 = 7,
    /// Fire every 256 ticks
    _256 = 8,
    /// Fire every 512 ticks
    _512 = 9,
    /// Fire every 1024 ticks
    _1024 = 10,
    /// Fire every 2048 ticks
    _2048 = 11,
    /// Fire every 4096 ticks
    _4096 = 12,
    /// Fire every 8192 ticks
    _8192 = 13,
    /// Fire every 16384 ticks
    _16384 = 14,
    /// Fire every 32768 ticks
    _32768 = 15,
}
