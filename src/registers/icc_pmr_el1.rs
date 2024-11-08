use tock_registers::{interfaces::*, register_bitfields};

register_bitfields! {u64,
    pub ICC_PMR_EL1 [
        PRIORITY OFFSET(0) NUMBITS(8) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ICC_PMR_EL1::Register;

    sys_coproc_read_raw!(u64, "ICC_PMR_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ICC_PMR_EL1::Register;

    sys_coproc_write_raw!(u64, "ICC_PMR_EL1", "x");
}

pub const ICC_PMR_EL1: Reg = Reg {};
