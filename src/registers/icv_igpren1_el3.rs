use tock_registers::{interfaces::*, register_bitfields};

register_bitfields! {u64,
    pub ICV_IGPREN1_EL3 [
        ENGRP1S OFFSET(1) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        ENGRP1NS OFFSET(0) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ICV_IGPREN1_EL3::Register;

    sys_coproc_read_raw!(u64, "ICV_IGPREN1_EL3", "x");
}

pub const ICV_IGPREN1_EL3: Reg = Reg {};
