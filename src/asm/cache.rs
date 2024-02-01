use crate::mmu::address::Segment;

mod sealed {

    use crate::mmu::address::Segment;

    pub trait Ic {
        fn ic(&self);
    }

    pub trait Dc {
        fn dc(&self, seg: Segment);
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
            fn dc(&self, mut seg: Segment){
                seg.round(64);
                match() {
                    #[cfg(target_arch = "aarch64")]
                    () => unsafe {
                        for i in(seg.0 .. seg.1).step_by(64){
                            core::arch::asm!(concat!("dc ",stringify!($A), ",{}"), in(reg) i, options(nostack))
                        }
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
dc!(CVAC, CVac);
dc!(IVAC, IVac);
dc!(CIVAC, CIVac);

#[inline(always)]
pub fn ic(_arg: impl sealed::Ic) {
    _arg.ic();
}

#[inline(always)]
pub fn dc(_arg: impl sealed::Dc, seg: Segment){
    _arg.dc(seg);
}
