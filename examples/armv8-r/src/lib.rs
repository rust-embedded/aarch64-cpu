#![no_std]

pub mod fmt;
pub mod gic;
mod timer;
mod uart;

use crate::gic::Gic;
pub use aarch64_cpu::generic_timer::El2PhysicalTimer;

pub use timer::PHYS_INT_ID;

/// Subset of FVP peripherals used by the examples
pub struct Peripherals {
    /// EL1 Physical Timer
    pub el2_phys_timer: El2PhysicalTimer,
    pub gic: Gic,
}

impl Peripherals {
    /// Takes owning handles to the available peripherals
    pub fn take() -> Self {
        let arm_fvp_base_pac::Peripherals {
            refclk_cntcontrol,
            gicd,
            gicr,
            // used by defmt logger; do not use it for anything else
            uart0: _,
            ..
        } = arm_fvp_base_pac::Peripherals::take().expect("peripherals have already been taken");

        let el2_phys_timer = timer::setup(refclk_cntcontrol);

        Self {
            gic: Gic::setup(gicd, gicr),
            el2_phys_timer,
        }
    }
}
