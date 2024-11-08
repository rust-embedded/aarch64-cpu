use tock_registers::{interfaces::*, register_bitfields};

register_bitfields! {u64,
    pub ICV_BPR1_EL1 [
        BP OFFSET(0) NUMBITS(3) []
    ]
}

pub struct Reg;

impl Writeable for Reg {
    type T = u64;
    type R = ICV_BPR1_EL1::Register;

    sys_coproc_write_raw!(u64, "ICV_BPR1_EL1", "x");
}

impl Readable for Reg {
    type T = u64;
    type R = ICV_BPR1_EL1::Register;

    sys_coproc_read_raw!(u64, "ICV_BPR1_EL1", "x");
}

pub const ICV_BPR1_EL1: Reg = Reg {};
