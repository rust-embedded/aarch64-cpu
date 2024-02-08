//! Hypervisor IPA Fault Address Register
//!
//! Holds the faulting IPA for some aborts on a stage 2 translation taken to EL2.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub HPFAR_EL2 [
        /// Faulting IPA address space
        NS   OFFSET(63) NUMBITS(1) [
            Secure = 0,
            NonSecure = 1
        ],
        /// Faulting Intermediate Physical Address
        /// 40 bits, 40 = 52 - 12. 12 is the size of the page.
        /// For implementations with fewer than 52 physical address bits, the corresponding upper bits in this field are RES0.
        FIPA OFFSET(4) NUMBITS(40) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = HPFAR_EL2::Register;

    sys_coproc_read_raw!(u64, "HPFAR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = HPFAR_EL2::Register;

    sys_coproc_write_raw!(u64, "HPFAR_EL2", "x");
}

pub const HPFAR_EL2: Reg = Reg {};
