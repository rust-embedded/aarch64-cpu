//! Interrupts

#[cfg(target_arch = "aarch64")]
use crate::registers::{Readable, Writeable, DAIF};
#[cfg(target_arch = "aarch64")]
use core::sync::atomic::{compiler_fence, Ordering};

/// Disables all interrupts in the current core.
#[cfg(target_arch = "aarch64")]
#[inline]
pub fn disable() {
    compiler_fence(Ordering::SeqCst);
    DAIF.write(DAIF::I::Masked + DAIF::F::Masked);
    compiler_fence(Ordering::SeqCst);
}

/// Enables all the interrupts in the current core.
///
/// # Safety
///
/// - Do not call this function inside a critical section.
#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn enable() {
    compiler_fence(Ordering::SeqCst);
    DAIF.write(DAIF::I::Unmasked + DAIF::F::Unmasked);
    compiler_fence(Ordering::SeqCst);
}

#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn enabled() -> bool {
    if DAIF.read(DAIF::I) == 0 || DAIF.read(DAIF::F) == 0 {
        return true;
    }
    false
}
