use core::ptr::NonNull;

use arm_fvp_base_pac::{PhysicalInstance, UniqueMmioPointer};
use arm_generic_timer::{CntControlBase, GenericTimerControl};
use arm_gic::IntId;

/// The interrupt ID tied to our physical timer
pub const PHYS_INT_ID: IntId = IntId::ppi(14);

/// Configure the EL2 timer
pub fn setup(
    refclk_cntcontrol: PhysicalInstance<CntControlBase>,
) -> aarch64_cpu::generic_timer::El2PhysicalTimer {
    let mut cnt_control_base = unsafe {
        GenericTimerControl::new(UniqueMmioPointer::new(
            NonNull::new(refclk_cntcontrol.pa() as *mut _).unwrap(),
        ))
    };
    // scaling is reported as not supported but disable it anyways
    cnt_control_base.disable_scaling();
    cnt_control_base.set_enable(true);
    semihosting::println!(
        "Timer frequency is reported as {} Hz",
        cnt_control_base.base_frequency()
    );

    // We were given the physical instance for the timer control block, so this is safe
    let mut timer = unsafe { aarch64_cpu::generic_timer::El2PhysicalTimer::new() };

    timer.frequency_hz_set(cnt_control_base.base_frequency());

    timer
}
