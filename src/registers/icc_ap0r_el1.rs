use tock_registers::register_bitfields;

register_bitfields! {u64,
    pub ICC_AP0R_EL1 [
        FIELDS OFFSET(0) NUMBITS(32) []
    ]
}

macro_rules! ap {
    ($reg: ident, $name: tt) => {
        #[allow(non_snake_case)]
        mod $reg {
            use super::*;
            use tock_registers::interfaces::*;
            pub struct Reg;
            impl Writeable for Reg {
                type T = u64;
                type R = ICC_AP0R_EL1::Register;
                sys_coproc_write_raw!(u64, $name, "x");
            }
            impl Readable for Reg {
                type T = u64;
                type R = ICC_AP0R_EL1::Register;
                sys_coproc_read_raw!(u64, $name, "x");
            }
        }
        pub const $reg: $reg::Reg = $reg::Reg {};
    };
}

ap!(ICC_AP0R0_EL1, "S3_0_C12_C9_4");
ap!(ICC_AP0R1_EL1, "S3_0_C12_C9_5");
ap!(ICC_AP0R2_EL1, "S3_0_C12_C9_6");
ap!(ICC_AP0R3_EL1, "S3_0_C12_C9_7");
