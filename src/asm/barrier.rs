// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Barrier functions.

mod sealed {
    pub trait Dmb {
        fn dmb(&self);
    }

    pub trait Dsb {
        fn dsb(&self);
    }

    pub trait Isb {
        fn isb(&self);
    }
}

macro_rules! dmb_dsb {
    ($A:ident, $T: ident) => {
        pub struct $T;
        pub const $A: $T = $T {};

        impl sealed::Dmb for $T {
            #[inline(always)]
            fn dmb(&self) {
                match () {
                    #[cfg(target_arch = "aarch64")]
                    () => unsafe {
                        core::arch::asm!(concat!("DMB ", stringify!($A)), options(nostack))
                    },

                    #[cfg(not(target_arch = "aarch64"))]
                    () => unimplemented!(),
                }
            }
        }
        impl sealed::Dsb for $T {
            #[inline(always)]
            fn dsb(&self) {
                match () {
                    #[cfg(target_arch = "aarch64")]
                    () => unsafe {
                        core::arch::asm!(concat!("DSB ", stringify!($A)), options(nostack))
                    },

                    #[cfg(not(target_arch = "aarch64"))]
                    () => unimplemented!(),
                }
            }
        }
    };
}

dmb_dsb!(SY, Sy);
dmb_dsb!(ST, St);
dmb_dsb!(LD, Ld);
dmb_dsb!(ISH, Ish);
dmb_dsb!(ISHST, Ishst);
dmb_dsb!(ISHLD, Ishld);
dmb_dsb!(NSH, Nsh);
dmb_dsb!(NSHST, Nshst);
dmb_dsb!(NSHLD, Nshld);
dmb_dsb!(OSH, Osh);
dmb_dsb!(OSHST, Oshst);
dmb_dsb!(OSHLD, Oshld);

impl sealed::Isb for Sy {
    #[inline(always)]
    fn isb(&self) {
        match () {
            #[cfg(target_arch = "aarch64")]
            () => unsafe { core::arch::asm!("ISB SY", options(nostack)) },

            #[cfg(not(target_arch = "aarch64"))]
            () => unimplemented!(),
        }
    }
}

pub fn isb(_arg: impl sealed::Isb) {
    _arg.isb()
}

pub fn dmb(_arg: impl sealed::Dmb) {
    _arg.dmb()
}

pub fn dsb(_arg: impl sealed::Dsb){
    _arg.dsb()
}
