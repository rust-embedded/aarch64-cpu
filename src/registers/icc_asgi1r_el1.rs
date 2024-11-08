use tock_registers::{interfaces::*, register_bitfields};

register_bitfields! {u64,
    pub ICC_ASGI1R_EL1 [
        Aff3  OFFSET(48) NUMBITS(8) [],
        RS    OFFSET(44) NUMBITS(4) [],
        IRM   OFFSET(40) NUMBITS(1) [],
        Aff2  OFFSET(32) NUMBITS(8) [

        ],
        INTID OFFSET(24) NUMBITS(4) [

        ],
        Aff1 OFFSET(16) NUMBITS(24) [

        ],
        TARGET OFFSET(0) NUMBITS(16) [

        ]
    ]
}

pub struct Reg;

impl Writeable for Reg {
    type T = u64;
    type R = ICC_ASGI1R_EL1::Register;

    sys_coproc_write_raw!(u64, "ICC_ASGI1R_EL1", "x");
}

impl Readable for Reg {
    type T = u64;
    type R = ICC_ASGI1R_EL1::Register;

    sys_coproc_read_raw!(u64, "ICC_ASGI1R_EL1", "x");
}

pub const ICC_ASGI1R_EL1: Reg = Reg {};
