use crate::registers::*;

mod sealed {
    pub trait At {
        fn at(&self, va: u64);
    }
}

macro_rules! at {
    ($A:ident, $T: ident) => {
        pub struct $T;
        pub const $A: $T = $T {};

        impl sealed::At for $T {
            #[inline(always)]
            fn at(&self, va: u64) {
                match () {
                    #[cfg(target_arch = "aarch64")]
                    () => unsafe {
                        core::arch::asm!(concat!("at ", stringify!($A), ", {0}"), in(reg) va, options(nomem, nostack))
                    },

                    #[cfg(not(target_arch = "aarch64"))]
                    () => unimplemented!(),
                }
            }
        }
    };
}

at!(S1E1R, S1e1r);
at!(S1E2R, S1e2r);

#[inline(always)]
pub fn at(_arg: impl sealed::At, va: u64) {
    _arg.at(va);
}

#[inline(always)]
pub fn translate(_arg: impl sealed::At, va: u64) -> Option<u64> {
    _arg.at(va);
    let par = PAR_EL1.get();
    if PAR_EL1::F.read(par) == PAR_EL1::F::TranslationAborted.value {
        None
    } else {
        Some(PAR_EL1::PA.read(par) << 12)
    }
}
