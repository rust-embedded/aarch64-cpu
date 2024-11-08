use tock_registers::{interfaces::*, register_bitfields};

register_bitfields! {u64,
    pub ICV_PMR_EL1 [
        PRIORITY OFFSET(0) NUMBITS(8) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ICV_PMR_EL1::Register;

    sys_coproc_read_raw!(u64, "ICV_PMR_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ICV_PMR_EL1::Register;

    sys_coproc_write_raw!(u64, "ICV_PMR_EL1", "x");
}

pub const ICV_PMR_EL1: Reg = Reg {};
