use tock_registers::register_bitfields;
use tock_registers::interfaces::*;

register_bitfields! {u64,
    pub ICH_EISR_EL2 [
        STATUS15 OFFSET(15) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        STATUS14 OFFSET(14) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        STATUS13 OFFSET(13) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        STATUS12 OFFSET(12) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        STATUS11 OFFSET(11) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        STATUS10 OFFSET(10) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        STATUS9 OFFSET(9) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        STATUS8 OFFSET(8) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        STATUS7 OFFSET(7) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        STATUS6 OFFSET(6) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        STATUS5 OFFSET(5) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        STATUS4 OFFSET(4) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        STATUS3 OFFSET(3) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        STATUS2 OFFSET(2) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        STATUS1 OFFSET(1) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        STATUS0 OFFSET(0) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ICH_EISR_EL2::Register;

    sys_coproc_read_raw!(u64, "ICH_EISR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ICH_EISR_EL2::Register;

    sys_coproc_write_raw!(u64, "ICH_EISR_EL2", "x");
}

pub const ICH_EISR_EL2: Reg = Reg {};
