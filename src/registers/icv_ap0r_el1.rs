use tock_registers::register_bitfields;

register_bitfields! {u64,
    pub ICV_AP0R_EL1 [
        FIELDS OFFSET(0) NUMBITS(32) []
    ]
}

macro_rules! ap {
    ($reg: ident, $name: tt) => {
        #[allow(non_snake_case)]
        mod $reg {
            use tock_registers::interfaces::*;
            use super::*;
            pub struct Reg;
            impl Writeable for Reg {
                type T = u64;
                type R = ICV_AP0R_EL1::Register;
                sys_coproc_write_raw!(u64, $name, "x");
            }
            impl Readable for Reg {
                type T = u64;
                type R = ICV_AP0R_EL1::Register;
                sys_coproc_read_raw!(u64, $name, "x");
            }
        }
        pub const $reg: $reg::Reg = $reg::Reg {};
    };
}

ap!(ICV_AP0R0_EL1, "ICV_AP0R0_EL1");
ap!(ICV_AP0R1_EL1, "ICV_AP0R1_EL1");
ap!(ICV_AP0R2_EL1, "ICV_AP0R2_EL1");
ap!(ICV_AP0R3_EL1, "ICV_AP0R3_EL1");
