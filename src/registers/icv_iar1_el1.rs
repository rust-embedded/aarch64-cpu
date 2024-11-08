use tock_registers::{interfaces::*, register_bitfields};

register_bitfields! {u64,
    pub ICV_IAR1_EL1 [
        INTID OFFSET(0) NUMBITS(24) []
    ]
}

pub struct Reg;

impl Writeable for Reg {
    type T = u64;
    type R = ICV_IAR1_EL1::Register;

    sys_coproc_write_raw!(u64, "ICV_IAR1_EL1", "x");
}

pub const ICV_IAR1_EL1: Reg = Reg {};
