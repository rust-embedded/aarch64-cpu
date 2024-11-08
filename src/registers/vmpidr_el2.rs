use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub VMPIDR_EL2 [
        U    OFFSET(32) NUMBITS(8),
        AFF3 OFFSET(25) NUMBITS(5),
        MT   OFFSET(24) NUMBITS(1),
        AFF2 OFFSET(18) NUMBITS(5),
        AFF1 OFFSET(8) NUMBITS(9),
        AFF0 OFFSET(0) NUMBITS(8)
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = VMPIDR_EL2::Register;

    sys_coproc_read_raw!(u64, "VMPIDR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = VMPIDR_EL2::Register;

    sys_coproc_write_raw!(u64, "VMPIDR_EL2", "x");
}

pub const VMPIDR_EL2: Reg = Reg {};
