use tock_registers::register_bitfields;

register_bitfields! {u64,
    pub ICC_AP1R_EL1 [
        FIELDS OFFSET(0) NUMBITS(32) []
    ]
}

macro_rules! ap {
    ($reg: ident, $asm_name: tt) => {
        #[allow(non_snake_case)]
        mod $reg {
            use super::*;
            use tock_registers::interfaces::*;
            pub struct Reg;
            impl Writeable for Reg {
                type T = u64;
                type R = ICC_AP1R_EL1::Register;
                sys_coproc_write_raw!(u64, $asm_name, "x");
            }
            impl Readable for Reg {
                type T = u64;
                type R = ICC_AP1R_EL1::Register;
                sys_coproc_read_raw!(u64, $asm_name, "x");
            }
        }
        pub const $reg: $reg::Reg = $reg::Reg {};
    };
}

ap!(ICC_AP1R0_EL1, "S3_0_C12_C9_0");
ap!(ICC_AP1R1_EL1, "S3_0_C12_C9_1");
ap!(ICC_AP1R2_EL1, "S3_0_C12_C9_2");
ap!(ICC_AP1R3_EL1, "S3_0_C12_C9_3");
