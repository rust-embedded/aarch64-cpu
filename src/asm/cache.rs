mod sealed {

    pub trait Ic {
        fn ic(&self);
    }

    pub trait Dc {
        fn dc(&self, addr: u64);
    }
}

macro_rules! ic {
    ($A:ident, $T: ident) => {
        pub struct $T;
        pub const $A: $T = $T {};

        impl sealed::Ic for $T {
            #[inline(always)]
            fn ic(&self) {
                match () {
                    #[cfg(target_arch = "aarch64")]
                    () => unsafe {
                        core::arch::asm!(concat!("ic ", stringify!($A)), options(nostack))
                    },

                    #[cfg(not(target_arch = "aarch64"))]
                    () => unimplemented!(),
                }
            }
        }
    };
}

macro_rules! dc {
    ($A: ident, $T: ident) => {
        pub struct $T;
        pub const $A: $T = $T{};
        impl sealed::Dc for $T {
            #[inline(always)]
            fn dc(&self, addr:u64){
                match() {
                    #[cfg(target_arch = "aarch64")]
                    () => unsafe {
                        core::arch::asm!(concat!("dc ",stringify!($A), ",{}"), in(reg) addr, options(nostack))
                    },
                    #[cfg(not(target_arch = "aarch64"))]
                    () => unimplemented!(),
                }
            }
        }
    }
}

ic!(IALLU, Iallu);
ic!(IALLUIS, Ialluis);
dc!(CVAC, Cvac);
dc!(IVAC, Ivac);
dc!(CIVAC, Civac);
dc!(CISW, Cisw);
dc!(ISW, Isw);
dc!(CSW, Csw);

#[inline(always)]
pub fn ic(_arg: impl sealed::Ic) {
    _arg.ic();
}

#[inline(always)]
pub fn dc(_arg: impl sealed::Dc, addr: u64) {
    _arg.dc(addr);
}
