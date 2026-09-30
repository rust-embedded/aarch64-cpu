//! Code and types for Generic Timer support at EL0 on Armv8-R.

use crate::registers;

use tock_registers::interfaces::{Readable, Writeable};

/// Represents our Physical Timer when we are running at EL0.
///
/// Note that for most of these APIs to work, EL0 needs to have been granted
/// access using methods like
/// [El1PhysicalTimer::el0_access_physical_counter](crate::generic_timer::El1PhysicalTimer::el0_access_physical_counter).
pub struct El0PhysicalTimer();

impl El0PhysicalTimer {
    /// Create a Physical Timer driver suitable for use at EL0
    ///
    /// EL2/EL1 has to grant permission for EL0 to use the Physical Timer, so
    /// check they did that otherwise using this driver will raise a hardware exception.
    ///
    /// # Safety
    ///
    /// Only create one of [`El0PhysicalTimer`], [`El1PhysicalTimer`](super::El1PhysicalTimer) or
    /// [`El2PhysicalTimer`](super::El2PhysicalTimer) at any given time, as they all access
    /// the same shared mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> El0PhysicalTimer {
        El0PhysicalTimer()
    }
}

impl super::GenericTimer for El0PhysicalTimer {
    fn frequency_hz(&self) -> u32 {
        registers::CNTFRQ_EL0.get() as u32
    }

    fn counter(&self) -> u64 {
        registers::CNTPCT_EL0.get()
    }

    fn counter_compare(&self) -> u64 {
        registers::CNTP_CVAL_EL0.get()
    }

    fn counter_compare_set(&mut self, value: u64) {
        registers::CNTP_CVAL_EL0.set(value)
    }

    fn countdown(&self) -> u32 {
        registers::CNTP_TVAL_EL0.get() as u32
    }

    fn countdown_set(&mut self, duration_ticks: u32) {
        registers::CNTP_TVAL_EL0.set(duration_ticks as u64)
    }

    fn enabled(&self) -> bool {
        registers::CNTP_CTL_EL0.read(registers::CNTP_CTL_EL0::ENABLE) != 0
    }

    fn enable(&self, enabled: bool) {
        registers::CNTP_CTL_EL0.write(if enabled {
            registers::CNTP_CTL_EL0::ENABLE::SET
        } else {
            registers::CNTP_CTL_EL0::ENABLE::CLEAR
        })
    }

    fn interrupt_masked(&self) -> bool {
        registers::CNTP_CTL_EL0.read(registers::CNTP_CTL_EL0::IMASK) != 0
    }

    fn interrupt_mask(&mut self, mask: bool) {
        registers::CNTP_CTL_EL0.write(if mask {
            registers::CNTP_CTL_EL0::IMASK::SET
        } else {
            registers::CNTP_CTL_EL0::IMASK::CLEAR
        })
    }

    fn interrupt_status(&self) -> bool {
        registers::CNTP_CTL_EL0.read(registers::CNTP_CTL_EL0::ISTATUS) != 0
    }
}

/// Represents our Virtual Timer when we are running at EL0.
///
/// Note that for most of these APIs to work, EL0 needs to have been granted
/// access using methods like
/// [El1VirtualTimer::el0_access_virtual_counter](crate::generic_timer::El1VirtualTimer::el0_access_virtual_counter).
pub struct El0VirtualTimer();

impl El0VirtualTimer {
    /// Create a Virtual Timer driver suitable for use at EL0
    ///
    /// EL2/EL1 has to grant permission for EL0 to use the Virtual Timer, so
    /// check they did that otherwise using this driver will raise a hardware exception.
    ///
    /// # Safety
    ///
    /// Only create one of [`El0VirtualTimer`], [`El1VirtualTimer`](super::El1VirtualTimer) or
    /// [`El2VirtualTimer`](super::El2VirtualTimer) at any given time, as they all access the
    /// same shared mutable state within the processor and do read-modify-writes on that state.
    pub unsafe fn new() -> El0VirtualTimer {
        El0VirtualTimer()
    }
}

impl super::GenericTimer for El0VirtualTimer {
    fn frequency_hz(&self) -> u32 {
        registers::CNTFRQ_EL0.get() as u32
    }

    fn counter(&self) -> u64 {
        registers::CNTVCT_EL0.get()
    }

    fn counter_compare(&self) -> u64 {
        registers::CNTV_CVAL_EL0.get()
    }

    fn counter_compare_set(&mut self, value: u64) {
        registers::CNTV_CVAL_EL0.set(value)
    }

    fn countdown(&self) -> u32 {
        registers::CNTV_TVAL_EL0.get() as u32
    }

    fn countdown_set(&mut self, duration_ticks: u32) {
        registers::CNTV_TVAL_EL0.set(duration_ticks as u64)
    }

    fn enabled(&self) -> bool {
        registers::CNTV_CTL_EL0.read(registers::CNTV_CTL_EL0::ENABLE) != 0
    }

    fn enable(&self, enabled: bool) {
        registers::CNTV_CTL_EL0.write(if enabled {
            registers::CNTV_CTL_EL0::ENABLE::SET
        } else {
            registers::CNTV_CTL_EL0::ENABLE::CLEAR
        })
    }

    fn interrupt_masked(&self) -> bool {
        registers::CNTV_CTL_EL0.read(registers::CNTV_CTL_EL0::IMASK) != 0
    }

    fn interrupt_mask(&mut self, mask: bool) {
        registers::CNTV_CTL_EL0.write(if mask {
            registers::CNTV_CTL_EL0::IMASK::SET
        } else {
            registers::CNTV_CTL_EL0::IMASK::CLEAR
        })
    }

    fn interrupt_status(&self) -> bool {
        registers::CNTV_CTL_EL0.read(registers::CNTV_CTL_EL0::ISTATUS) != 0
    }
}
