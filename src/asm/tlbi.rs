// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - ZhiHong Niu <zhihong@nzhnb.com>

//! TLB Invalidate (TLBI) instructions.
//!
//! Provides Rust wrappers around AArch64 TLBI system instructions for
//! invalidating TLB entries at EL1.
//!
//! # Operand Encoding
//!
//! For address-based TLBI operations (`vae1`, `vale1`, `vaae1`, `vaale1`),
//! the `u64` operand is encoded as specified in the Arm ARM:
//!
//! - Bits \[55:48\]: ASID (for non-"AA" variants)
//! - Bits \[43:0\]: VA\[55:12\] (the virtual address shifted right by 12)
//!
//! It is the caller's responsibility to construct the operand correctly.
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
//! // A DSB before ensures prior stores to page tables are visible;
//! // a DSB+ISB after ensures the invalidation completes before
//! // subsequent memory accesses.
//! barrier::dsb(barrier::SY);
//! tlbi::vmalle1();
//! barrier::dsb(barrier::SY);
//! barrier::isb(barrier::SY);
//!
//! // Invalidate by VA: encode operand as VA[55:12] | (ASID << 48).
//! let vaddr: u64 = 0x8000_0000;
//! let asid: u64 = 1;
//! let operand = (vaddr >> 12) | (asid << 48);
//! barrier::dsb(barrier::SY);
//! tlbi::vae1(operand);
//! barrier::dsb(barrier::SY);
//! barrier::isb(barrier::SY);
//! ```

/// Invalidate all EL1&0 regime TLB entries in the current VMID.
///
/// Executes `TLBI VMALLE1`.
#[inline(always)]
pub fn vmalle1() {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { core::arch::asm!("TLBI VMALLE1", options(nostack)) },

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}

/// Invalidate all EL1&0 regime TLB entries in the current VMID
/// on all PEs in the same Inner Shareable domain.
///
/// Executes `TLBI VMALLE1IS`.
#[inline(always)]
pub fn vmalle1is() {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { core::arch::asm!("TLBI VMALLE1IS", options(nostack)) },

        #[cfg(not(target_arch = "aarch64"))]
        () => unimplemented!(),
    }
}

/// Invalidate TLB entries by VA, EL1&0, current VMID.
///
/// Executes `TLBI VAE1, <Xt>`.
///
/// # Operand Encoding
///
/// - Bits \[55:48\]: ASID
/// - Bits \[43:0\]: VA\[55:12\]
#[inline(always)]
pub fn vae1(val: u64) {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { core::arch::asm!("TLBI VAE1, {v}", v = in(reg) val, options(nostack)) },

        #[cfg(not(target_arch = "aarch64"))]
        () => {
            let _ = val;
            unimplemented!()
        }
    }
}

/// Invalidate TLB entries by VA, EL1&0, current VMID,
/// Inner Shareable.
///
/// Executes `TLBI VAE1IS, <Xt>`.
///
/// # Operand Encoding
///
/// - Bits \[55:48\]: ASID
/// - Bits \[43:0\]: VA\[55:12\]
#[inline(always)]
pub fn vae1is(val: u64) {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { core::arch::asm!("TLBI VAE1IS, {v}", v = in(reg) val, options(nostack)) },

        #[cfg(not(target_arch = "aarch64"))]
        () => {
            let _ = val;
            unimplemented!()
        }
    }
}

/// Invalidate TLB entries by VA, last level, EL1&0, current VMID.
///
/// Executes `TLBI VALE1, <Xt>`.
///
/// Only invalidates entries from the last level of the translation
/// table walk (leaf entries), which can be more efficient than `vae1`
/// when intermediate entries are known to be unaffected.
///
/// # Operand Encoding
///
/// - Bits \[55:48\]: ASID
/// - Bits \[43:0\]: VA\[55:12\]
#[inline(always)]
pub fn vale1(val: u64) {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { core::arch::asm!("TLBI VALE1, {v}", v = in(reg) val, options(nostack)) },

        #[cfg(not(target_arch = "aarch64"))]
        () => {
            let _ = val;
            unimplemented!()
        }
    }
}

/// Invalidate TLB entries by VA, last level, EL1&0, current VMID,
/// Inner Shareable.
///
/// Executes `TLBI VALE1IS, <Xt>`.
///
/// # Operand Encoding
///
/// - Bits \[55:48\]: ASID
/// - Bits \[43:0\]: VA\[55:12\]
#[inline(always)]
pub fn vale1is(val: u64) {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { core::arch::asm!("TLBI VALE1IS, {v}", v = in(reg) val, options(nostack)) },

        #[cfg(not(target_arch = "aarch64"))]
        () => {
            let _ = val;
            unimplemented!()
        }
    }
}

/// Invalidate TLB entries by VA, all ASIDs, EL1&0, current VMID.
///
/// Executes `TLBI VAAE1, <Xt>`.
///
/// Invalidates entries matching the VA regardless of ASID.
///
/// # Operand Encoding
///
/// - Bits \[43:0\]: VA\[55:12\]
#[inline(always)]
pub fn vaae1(val: u64) {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { core::arch::asm!("TLBI VAAE1, {v}", v = in(reg) val, options(nostack)) },

        #[cfg(not(target_arch = "aarch64"))]
        () => {
            let _ = val;
            unimplemented!()
        }
    }
}

/// Invalidate TLB entries by VA, all ASIDs, EL1&0, current VMID,
/// Inner Shareable.
///
/// Executes `TLBI VAAE1IS, <Xt>`.
///
/// # Operand Encoding
///
/// - Bits \[43:0\]: VA\[55:12\]
#[inline(always)]
pub fn vaae1is(val: u64) {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { core::arch::asm!("TLBI VAAE1IS, {v}", v = in(reg) val, options(nostack)) },

        #[cfg(not(target_arch = "aarch64"))]
        () => {
            let _ = val;
            unimplemented!()
        }
    }
}

/// Invalidate TLB entries by VA, all ASIDs, last level, EL1&0,
/// current VMID.
///
/// Executes `TLBI VAALE1, <Xt>`.
///
/// # Operand Encoding
///
/// - Bits \[43:0\]: VA\[55:12\]
#[inline(always)]
pub fn vaale1(val: u64) {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { core::arch::asm!("TLBI VAALE1, {v}", v = in(reg) val, options(nostack)) },

        #[cfg(not(target_arch = "aarch64"))]
        () => {
            let _ = val;
            unimplemented!()
        }
    }
}

/// Invalidate TLB entries by VA, all ASIDs, last level, EL1&0,
/// current VMID, Inner Shareable.
///
/// Executes `TLBI VAALE1IS, <Xt>`.
///
/// # Operand Encoding
///
/// - Bits \[43:0\]: VA\[55:12\]
#[inline(always)]
pub fn vaale1is(val: u64) {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { core::arch::asm!("TLBI VAALE1IS, {v}", v = in(reg) val, options(nostack)) },

        #[cfg(not(target_arch = "aarch64"))]
        () => {
            let _ = val;
            unimplemented!()
        }
    }
}

/// Invalidate TLB entries by ASID, EL1&0, current VMID.
///
/// Executes `TLBI ASIDE1, <Xt>`.
///
/// # Operand Encoding
///
/// - Bits \[55:48\]: ASID
#[inline(always)]
pub fn aside1(val: u64) {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { core::arch::asm!("TLBI ASIDE1, {v}", v = in(reg) val, options(nostack)) },

        #[cfg(not(target_arch = "aarch64"))]
        () => {
            let _ = val;
            unimplemented!()
        }
    }
}

/// Invalidate TLB entries by ASID, EL1&0, current VMID,
/// Inner Shareable.
///
/// Executes `TLBI ASIDE1IS, <Xt>`.
///
/// # Operand Encoding
///
/// - Bits \[55:48\]: ASID
#[inline(always)]
pub fn aside1is(val: u64) {
    match () {
        #[cfg(target_arch = "aarch64")]
        () => unsafe { core::arch::asm!("TLBI ASIDE1IS, {v}", v = in(reg) val, options(nostack)) },

        #[cfg(not(target_arch = "aarch64"))]
        () => {
            let _ = val;
            unimplemented!()
        }
    }
}
