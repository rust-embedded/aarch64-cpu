use tock_registers::{interfaces::*, register_bitfields};

register_bitfields! {u64,
    pub ICC_BPR1_EL1 [
        BP OFFSET(0) NUMBITS(3) []
    ]
}

pub struct Reg;

impl Writeable for Reg {
    type T = u64;
    type R = ICC_BPR1_EL1::Register;

    sys_coproc_write_raw!(u64, "ICC_BPR1_EL1", "x");
}

impl Readable for Reg {
    type T = u64;
    type R = ICC_BPR1_EL1::Register;

    sys_coproc_read_raw!(u64, "ICC_BPR1_EL1", "x");
}

pub const ICC_BPR1_EL1: Reg = Reg {};
