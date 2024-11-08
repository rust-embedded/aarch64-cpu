use tock_registers::{interfaces::*, register_bitfields};

register_bitfields! {u64,
    pub ICV_HPPIR0_EL1 [
        INTID OFFSET(0) NUMBITS(24) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ICV_HPPIR0_EL1::Register;

    sys_coproc_read_raw!(u64, "ICV_HPPIR0_EL1", "x");
}

pub const ICV_HPPIR0_EL1: Reg = Reg {};
