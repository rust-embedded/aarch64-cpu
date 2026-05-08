#![no_std]

pub mod fmt;
pub mod gic;
mod uart;

use crate::gic::Gic;

/// Subset of FVP peripherals used by the examples
pub struct Peripherals {
    pub gic: Gic,
}

impl Peripherals {
    /// Takes owning handles to the available peripherals
    pub fn take() -> Self {
        let arm_fvp_base_pac::Peripherals {
            gicd, gicr, uart0, ..
        } = arm_fvp_base_pac::Peripherals::take().expect("peripherals have already been taken");
        // used by defmt logger; do not use it for anything else
        drop(uart0);

        Self {
            gic: Gic::setup(gicd, gicr),
        }
    }
}
