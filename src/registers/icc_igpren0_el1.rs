use tock_registers::{interfaces::*, register_bitfields};

register_bitfields! {u64,
    pub ICC_IGPREN0_EL1 [
        EN OFFSET(0) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ICC_IGPREN0_EL1::Register;

    sys_coproc_read_raw!(u64, "ICC_IGPREN0_EL1", "x");
}

pub const ICC_IGPREN0_EL1: Reg = Reg {};
