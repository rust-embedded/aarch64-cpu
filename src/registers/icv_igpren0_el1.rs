use tock_registers::{interfaces::*, register_bitfields};

register_bitfields! {u64,
    pub ICV_IGPREN0_EL1 [
        EN OFFSET(0) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ICV_IGPREN0_EL1::Register;

    sys_coproc_read_raw!(u64, "ICV_IGPREN0_EL1", "x");
}

pub const ICV_IGPREN0_EL1: Reg = Reg {};
