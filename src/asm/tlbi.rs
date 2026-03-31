// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2026 by the author(s)
//
// Author(s):
//   - ZhiHong Niu <zhihong@nzhnb.com>

//! TLB Invalidate (TLBI) instructions.
//!
//! Provides Rust wrappers around AArch64 TLBI system instructions for
//! invalidating TLB entries at EL1.
//!
//! # Operand Types
//!
//! | Type | Fields | Used by |
//! |------|--------|---------|
//! | [`Addr`] | VA and ASID | `vae1`, `vale1`, `vaae1`, `vaale1` + IS variants |
//! | [`Asid`] | ASID only | `aside1` + IS variant |
//!
//! # Synchronization
//!
//! TLBI instructions must be surrounded by appropriate barrier instructions
//! (DSB before and DSB+ISB after) to ensure the invalidation takes effect.
//! See [`barrier`](super::barrier).
//!
//! # References
//!
//! - [Arm ARM §C5.5 - A64 system instructions for TLB maintenance](https://developer.arm.com/documentation/ddi0487/latest)
//! - [Arm ARM §D8.10 - TLB maintenance instructions](https://developer.arm.com/documentation/ddi0487/latest)
//!
//! # Example
//!
//! ```no_run
//! use aarch64_cpu::asm::{barrier, tlbi};
//!
//! // Invalidate all EL1&0 regime TLB entries.
//! barrier::dsb(barrier::SY);
//! tlbi::vmalle1();
//! barrier::dsb(barrier::SY);
//! barrier::isb(barrier::SY);
//!
//! // Invalidate by VA + ASID.
//! barrier::dsb(barrier::SY);
//! tlbi::vae1(tlbi::Addr::new(0x8000_0000, 1));
//! barrier::dsb(barrier::SY);
//! barrier::isb(barrier::SY);
//!
//! // Invalidate by VA, all ASIDs (pass ASID = 0).
//! barrier::dsb(barrier::SY);
//! tlbi::vaae1(tlbi::Addr::new(0x8000_0000, 0));
//! barrier::dsb(barrier::SY);
//! barrier::isb(barrier::SY);
//!
//! // Invalidate by ASID.
//! barrier::dsb(barrier::SY);
//! tlbi::aside1(tlbi::Asid::new(1));
//! barrier::dsb(barrier::SY);
//! barrier::isb(barrier::SY);
//! ```
//!
//! # Future Work
//!
//! - FEAT_TTL (ARMv8.4): Allow setting the TTL hint in bits \[47:44\] of
//!   [`Addr`] for more efficient invalidation when the page table level
//!   is known.
//! - FEAT_TLBIRANGE (ARMv8.4): Add range-based TLBI instructions
//!   (`RVAE1`, `RVALE1`, etc.) with their distinct operand encoding.
//! - Outer Shareable variants (ARMv8.4): `VMALLE1OS`, `VAE1OS`,
//!   `VALE1OS`, etc.

/// Encoded TLBI operand carrying a virtual address and ASID.
///
/// Layout per Arm ARM:
/// - Bits \[43:0\]: VA\[55:12\] (virtual address shifted right by 12)
/// - Bits \[47:44\]: RES0 (or TTL hint when FEAT_TTL is implemented)
/// - Bits \[63:48\]: ASID (8 or 16 bits depending on `TCR_EL1.AS`)
///
/// For use with [`vae1`], [`vae1is`], [`vale1`], [`vale1is`],
/// [`vaae1`], [`vaae1is`], [`vaale1`], [`vaale1is`].
///
/// For all-ASID variants (`vaae1`, `vaale1`, etc.) where the ASID
/// field is ignored by hardware, pass `asid = 0`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Addr(u64);

impl Addr {
    /// VA field mask: bits [43:0].
    const VA_MASK: u64 = 0x0000_0FFF_FFFF_FFFF;

    /// Encode from a virtual address and ASID.
    ///
    /// The low 12 bits of `va` are discarded (the hardware operates on
    /// page-number granularity).
    #[inline(always)]
    pub const fn new(va: u64, asid: u16) -> Self {
        Self(((va >> 12) & Self::VA_MASK) | ((asid as u64) << 48))
    }

    /// Construct from a pre-encoded raw value.
    #[inline(always)]
    pub const fn from_raw(val: u64) -> Self {
        Self(val)
    }

    /// Return the raw encoded value.
    #[inline(always)]
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

/// Encoded TLBI operand carrying an ASID only.
///
/// Layout per Arm ARM:
/// - Bits \[47:0\]: RES0
/// - Bits \[63:48\]: ASID (8 or 16 bits depending on `TCR_EL1.AS`)
///
/// For use with [`aside1`], [`aside1is`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Asid(u64);

impl Asid {
    /// Encode from an ASID value.
    #[inline(always)]
    pub const fn new(asid: u16) -> Self {
        Self((asid as u64) << 48)
    }

    /// Construct from a pre-encoded raw value.
    #[inline(always)]
    pub const fn from_raw(val: u64) -> Self {
        Self(val)
    }

    /// Return the raw encoded value.
    #[inline(always)]
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

// Generate TLBI instruction wrappers that take no operand.
macro_rules! tlbi_no_arg {
    ($($(#[$attr:meta])* $fn_name:ident, $inst:tt;)*) => {
        $(
            $(#[$attr])*
            #[inline(always)]
            pub fn $fn_name() {
                #[cfg(target_arch = "aarch64")]
                unsafe {
                    core::arch::asm!(concat!("TLBI ", stringify!($inst)), options(nostack))
                }

                #[cfg(not(target_arch = "aarch64"))]
                unimplemented!()
            }
        )*
    };
}

// Generate TLBI instruction wrappers that take a typed operand.
macro_rules! tlbi_arg {
    ($($(#[$attr:meta])* $fn_name:ident, $inst:tt, $arg_type:ty, $arg_name:ident;)*) => {
        $(
            $(#[$attr])*
            #[inline(always)]
            pub fn $fn_name($arg_name: $arg_type) {
                #[cfg(target_arch = "aarch64")]
                unsafe {
                    core::arch::asm!(
                        concat!("TLBI ", stringify!($inst), ", {v}"),
                        v = in(reg) $arg_name.as_raw(),
                        options(nostack),
                    )
                }

                #[cfg(not(target_arch = "aarch64"))]
                {
                    let _ = $arg_name;
                    unimplemented!()
                }
            }
        )*
    };
}

tlbi_no_arg! {
    /// Invalidate all EL1&0 regime TLB entries in the current VMID.
    ///
    /// Executes `TLBI VMALLE1`.
    vmalle1, VMALLE1;

    /// Invalidate all EL1&0 regime TLB entries in the current VMID
    /// on all PEs in the same Inner Shareable domain.
    ///
    /// Executes `TLBI VMALLE1IS`.
    vmalle1is, VMALLE1IS;
}

tlbi_arg! {
    /// Invalidate TLB entries by VA, EL1&0, current VMID.
    ///
    /// Executes `TLBI VAE1, <Xt>`.
    vae1, VAE1, Addr, addr;

    /// Invalidate TLB entries by VA, EL1&0, current VMID,
    /// Inner Shareable.
    ///
    /// Executes `TLBI VAE1IS, <Xt>`.
    vae1is, VAE1IS, Addr, addr;

    /// Invalidate TLB entries by VA, last level, EL1&0, current VMID.
    ///
    /// Executes `TLBI VALE1, <Xt>`.
    ///
    /// Only invalidates entries from the last level of the translation
    /// table walk (leaf entries), which can be more efficient than [`vae1`]
    /// when intermediate entries are known to be unaffected.
    vale1, VALE1, Addr, addr;

    /// Invalidate TLB entries by VA, last level, EL1&0, current VMID,
    /// Inner Shareable.
    ///
    /// Executes `TLBI VALE1IS, <Xt>`.
    vale1is, VALE1IS, Addr, addr;

    /// Invalidate TLB entries by VA, all ASIDs, EL1&0, current VMID.
    ///
    /// Executes `TLBI VAAE1, <Xt>`.
    ///
    /// Invalidates entries matching the VA regardless of ASID.
    vaae1, VAAE1, Addr, addr;

    /// Invalidate TLB entries by VA, all ASIDs, EL1&0, current VMID,
    /// Inner Shareable.
    ///
    /// Executes `TLBI VAAE1IS, <Xt>`.
    vaae1is, VAAE1IS, Addr, addr;

    /// Invalidate TLB entries by VA, all ASIDs, last level, EL1&0,
    /// current VMID.
    ///
    /// Executes `TLBI VAALE1, <Xt>`.
    vaale1, VAALE1, Addr, addr;

    /// Invalidate TLB entries by VA, all ASIDs, last level, EL1&0,
    /// current VMID, Inner Shareable.
    ///
    /// Executes `TLBI VAALE1IS, <Xt>`.
    vaale1is, VAALE1IS, Addr, addr;

    /// Invalidate TLB entries by ASID, EL1&0, current VMID.
    ///
    /// Executes `TLBI ASIDE1, <Xt>`.
    aside1, ASIDE1, Asid, asid;

    /// Invalidate TLB entries by ASID, EL1&0, current VMID,
    /// Inner Shareable.
    ///
    /// Executes `TLBI ASIDE1IS, <Xt>`.
    aside1is, ASIDE1IS, Asid, asid;
}
