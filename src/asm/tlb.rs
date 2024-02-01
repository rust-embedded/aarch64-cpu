mod sealed {
    pub trait Tlbi {
        fn invalidate(&self);
    }
}

macro_rules! tlbi{
    ($A: ident) => {
        pub struct $A;
        impl sealed::Tlbi for $A {
            #[inline(always)]
            fn invalidate(&self){
                match () {
                    #[cfg(target_arch = "aarch64")]
                    () => unsafe {
                        core::arch::asm!(concat!("TLBI ", stringify!($A)), options(nostack))
                    },

                    #[cfg(not(target_arch = "aarch64"))]
                    () => unimplemented!(),
                }
            }
        }
    }
}


tlbi!(VMALLE1IS);
tlbi!(ALLE2IS);
tlbi!(ALLE3IS);

#[inline(always)]
pub fn tlb_inv(_arg: impl sealed::Tlbi){
    _arg.invalidate();
}
