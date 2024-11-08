use tock_registers::register_bitfields;

register_bitfields! {u64,
    pub ICH_LR_EL2 [
        State   OFFSET(62) NUMBITS(2)  [],
        HW      OFFSET(61) NUMBITS(1)  [],
        Group   OFFSET(60) NUMBITS(1)  [],
        NMI     OFFSET(59) NUMBITS(1)  [],
        Priority OFFSET(48) NUMBITS(8) [],
        PINTID OFFSET(32) NUMBITS(13) [],
        VINTID OFFSET(0) NUMBITS(32) []
    ]
}

macro_rules! lr {
    ($reg: ident, $name: tt) => {
        #[allow(non_snake_case)]
        mod $reg {
            use super::*;
            use tock_registers::interfaces::*;
            pub struct Reg;
            impl Writeable for Reg {
                type T = u64;
                type R = ICH_LR_EL2::Register;
                sys_coproc_write_raw!(u64, $name, "x");
            }
            impl Readable for Reg {
                type T = u64;
                type R = ICH_LR_EL2::Register;
                sys_coproc_read_raw!(u64, $name, "x");
            }
        }
        pub const $reg: $reg::Reg = $reg::Reg {};
    };
}

lr!(ICH_LR0_EL2, "ICH_LR1_EL2");
lr!(ICH_LR1_EL2, "ICH_LR1_EL2");
lr!(ICH_LR2_EL2, "ICH_LR2_EL2");
lr!(ICH_LR3_EL2, "ICH_LR3_EL2");
lr!(ICH_LR4_EL2, "ICH_LR4_EL2");
lr!(ICH_LR5_EL2, "ICH_LR5_EL2");
lr!(ICH_LR6_EL2, "ICH_LR6_EL2");
lr!(ICH_LR7_EL2, "ICH_LR7_EL2");
lr!(ICH_LR8_EL2, "ICH_LR8_EL2");
lr!(ICH_LR9_EL2, "ICH_LR9_EL2");
lr!(ICH_LR10_EL2, "ICH_LR10_EL2");
lr!(ICH_LR11_EL2, "ICH_LR11_EL2");
lr!(ICH_LR12_EL2, "ICH_LR12_EL2");
lr!(ICH_LR13_EL2, "ICH_LR13_EL2");
lr!(ICH_LR14_EL2, "ICH_LR14_EL2");
lr!(ICH_LR15_EL2, "ICH_LR15_EL2");
