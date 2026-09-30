//! Code and types for Generic Timer support at EL2 on Armv8-R.

use super::{El1PhysicalTimer, El1VirtualTimer, GenericTimer};
use crate::registers;
use tock_registers::interfaces::{Readable, Writeable};

/// Represents our Physical Timer when we are running at EL2.
pub struct El2PhysicalTimer(El1PhysicalTimer);

impl El2PhysicalTimer {
    /// Create a Physical Timer driver suitable for use at EL2.
    ///
    /// # Safety
    ///
    /// Only create one of [`El2PhysicalTimer`], [`El1PhysicalTimer`](super::El1PhysicalTimer) or
    /// [`El0PhysicalTimer`](super::El0PhysicalTimer) at any given time, as they all access
    /// the same shared mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> El2PhysicalTimer {
        unsafe { El2PhysicalTimer(El1PhysicalTimer::new()) }
    }

    /// Set frequency
    ///
    /// Sets the frequency, in Hz, that the counters are incrementing at. You
    /// might need to call this if your system doesn't initialise the frequency
    /// value to something appropriate, or if you change the clock speed of the
    /// timer.
    pub fn frequency_hz_set(&mut self, new_frequency_hz: u32) {
        registers::CNTFRQ_EL0.set(new_frequency_hz as u64)
    }
}

impl GenericTimer for El2PhysicalTimer {
    fn frequency_hz(&self) -> u32 {
        self.0.frequency_hz()
    }

    fn counter(&self) -> u64 {
        self.0.counter()
    }

    fn counter_compare(&self) -> u64 {
        self.0.counter_compare()
    }

    fn counter_compare_set(&mut self, value: u64) {
        self.0.counter_compare_set(value)
    }

    fn countdown(&self) -> u32 {
        self.0.countdown()
    }

    fn countdown_set(&mut self, duration_ticks: u32) {
        self.0.countdown_set(duration_ticks)
    }

    fn enabled(&self) -> bool {
        self.0.enabled()
    }

    fn enable(&self, enabled: bool) {
        self.0.enable(enabled)
    }

    fn interrupt_masked(&self) -> bool {
        self.0.interrupt_masked()
    }

    fn interrupt_mask(&mut self, mask: bool) {
        self.0.interrupt_mask(mask)
    }

    fn interrupt_status(&self) -> bool {
        self.0.interrupt_status()
    }
}

/// Represents our Virtual Timer when we are running at EL2.
pub struct El2VirtualTimer(El1VirtualTimer);

impl El2VirtualTimer {
    /// Create a Virtual Timer driver suitable for use at EL2.
    ///
    /// # Safety
    ///
    /// Only create one of [`El2VirtualTimer`], [`El1VirtualTimer`](super::El1VirtualTimer) or
    /// [`El0VirtualTimer`](super::El0VirtualTimer) at any given time, as they all access the
    /// same shared mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> El2VirtualTimer {
        unsafe { El2VirtualTimer(El1VirtualTimer::new()) }
    }

    /// Set frequency
    ///
    /// Sets the frequency, in Hz, that the counters are incrementing at. You
    /// might need to call this if your system doesn't initialise the frequency
    /// value to something appropriate, or if you change the clock speed of the
    /// timer.
    pub fn frequency_hz_set(&mut self, new_frequency_hz: u32) {
        registers::CNTFRQ_EL0.set(new_frequency_hz as u64)
    }
}

impl GenericTimer for El2VirtualTimer {
    fn frequency_hz(&self) -> u32 {
        self.0.frequency_hz()
    }

    fn counter(&self) -> u64 {
        self.0.counter()
    }

    fn counter_compare(&self) -> u64 {
        self.0.counter_compare()
    }

    fn counter_compare_set(&mut self, value: u64) {
        self.0.counter_compare_set(value)
    }

    fn countdown(&self) -> u32 {
        self.0.countdown()
    }

    fn countdown_set(&mut self, duration_ticks: u32) {
        self.0.countdown_set(duration_ticks)
    }

    fn enabled(&self) -> bool {
        self.0.enabled()
    }

    fn enable(&self, enabled: bool) {
        self.0.enable(enabled)
    }

    fn interrupt_masked(&self) -> bool {
        self.0.interrupt_masked()
    }

    fn interrupt_mask(&mut self, mask: bool) {
        self.0.interrupt_mask(mask)
    }

    fn interrupt_status(&self) -> bool {
        self.0.interrupt_status()
    }
}

/// Represents our Hypervisor-specific Physical Timer when we are running at EL2.
pub struct El2HypPhysicalTimer();

impl El2HypPhysicalTimer {
    /// Create a Generic Timer driver for the EL2-specific Hyp(ervisor) Physical Timer.
    ///
    /// This timer hardware is distinct from the ElxPhysicalTimer and the ElxVirtualTimer.
    ///
    /// # Safety
    ///
    /// Only create one of these at any given time, as they access shared
    /// mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> El2HypPhysicalTimer {
        El2HypPhysicalTimer()
    }
}

impl super::GenericTimer for El2HypPhysicalTimer {
    fn frequency_hz(&self) -> u32 {
        registers::CNTFRQ_EL0.get() as u32
    }

    fn counter(&self) -> u64 {
        registers::CNTPCT_EL0.get()
    }

    fn counter_compare(&self) -> u64 {
        registers::CNTHP_CVAL_EL2.get()
    }

    fn counter_compare_set(&mut self, value: u64) {
        registers::CNTHP_CVAL_EL2.set(value)
    }

    fn countdown(&self) -> u32 {
        registers::CNTHP_TVAL_EL2.get() as u32
    }

    fn countdown_set(&mut self, duration_ticks: u32) {
        registers::CNTHP_TVAL_EL2.set(duration_ticks as u64)
    }

    fn enabled(&self) -> bool {
        registers::CNTHP_CTL_EL2.read(registers::CNTHP_CTL_EL2::ENABLE) != 0
    }

    fn enable(&self, enabled: bool) {
        registers::CNTHP_CTL_EL2.write(if enabled {
            registers::CNTHP_CTL_EL2::ENABLE::SET
        } else {
            registers::CNTHP_CTL_EL2::ENABLE::CLEAR
        })
    }

    fn interrupt_masked(&self) -> bool {
        registers::CNTHP_CTL_EL2.read(registers::CNTHP_CTL_EL2::IMASK) != 0
    }

    fn interrupt_mask(&mut self, mask: bool) {
        registers::CNTHP_CTL_EL2.write(if mask {
            registers::CNTHP_CTL_EL2::IMASK::SET
        } else {
            registers::CNTHP_CTL_EL2::IMASK::CLEAR
        })
    }

    fn interrupt_status(&self) -> bool {
        registers::CNTHP_CTL_EL2.read(registers::CNTHP_CTL_EL2::ISTATUS) != 0
    }
}
