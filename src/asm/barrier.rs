// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Barrier functions.

mod sealed {
    pub trait Dmb {
        fn dmb();
    }

    pub trait Dsb {
        fn dsb();
    }

    pub trait Isb {
        fn isb();
    }

    pub trait Tlbi {
        fn __tlbi();
    }
}

macro_rules! dmb_dsb {
    ($A:ident) => {
        impl sealed::Dmb for $A {
            #[inline(always)]
            fn dmb() {
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
        impl sealed::Dsb for $A {
            #[inline(always)]
            fn dsb() {
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




pub struct SY;
pub struct ST;
pub struct LD;
pub struct ISH;
pub struct ISHST;
pub struct ISHLD;
pub struct NSH;
pub struct NSHST;
pub struct NSHLD;
pub struct OSH;
pub struct OSHST;
pub struct OSHLD;

dmb_dsb!(SY);
dmb_dsb!(ST);
dmb_dsb!(LD);
dmb_dsb!(ISH);
dmb_dsb!(ISHST);
dmb_dsb!(ISHLD);
dmb_dsb!(NSH);
dmb_dsb!(NSHST);
dmb_dsb!(NSHLD);
dmb_dsb!(OSH);
dmb_dsb!(OSHST);
dmb_dsb!(OSHLD);

impl sealed::Isb for SY {
    #[inline(always)]
    fn isb() {
        match () {
            #[cfg(target_arch = "aarch64")]
            () => unsafe { core::arch::asm!("ISB SY", options(nostack)) },

            #[cfg(not(target_arch = "aarch64"))]
            () => unimplemented!(),
        }
    }
}

