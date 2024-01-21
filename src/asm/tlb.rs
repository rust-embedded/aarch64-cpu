mod sealed {
    pub trait Tlbi {
        fn invalidate();
    }
}

macro_rules! tlbi{
    ($A: ident) => {
        pub struct $A;
        impl sealed::Tlbi for $A {
            #[inline(always)]
            fn invalidate(){
                match () {
                    #[cfg(target_arch = "aarch64")]
                    () => unsafe {
                        core::arch::asm!(concat!("tlbi ", stringify!($A)), options(nostack))
                    },

                    #[cfg(not(target_arch = "aarch64"))]
                    () => unimplemented!(),
                }
            }
        }
    }
}


tlbi!(VMALLE1IS);
tlbi!(VMALLE2IS);
tlbi!(VMALLE3IS);
