#[cfg(feature = "critical-section-single-core")]
mod single_core {
    use core::sync::atomic;

    use crate::registers::{self, Readable, Writeable};

    struct SingleCoreCriticalSection;

    critical_section::set_impl!(SingleCoreCriticalSection);

    const DAIF_OFFSET: usize = 6;

    unsafe impl critical_section::Impl for SingleCoreCriticalSection {
        unsafe fn acquire() -> critical_section::RawRestoreState {
            // get which of DAIF are currently masked (shift to fit into `u8`)
            let daif_bits = registers::DAIF.get() >> DAIF_OFFSET;

            // mask everything (even if already masked)
            registers::DAIFSet.set(0b1111);

            // prevent reordering across the preceding register write
            atomic::compiler_fence(atomic::Ordering::SeqCst);

            // record which DAIF bits we need to clear later (only clear the ones that were not
            // masked)
            let bits_to_clear = daif_bits ^ 0b1111;
            bits_to_clear as u8
        }

        unsafe fn release(bits_to_clear: critical_section::RawRestoreState) {
            // prevent reordering across the following register write
            atomic::compiler_fence(atomic::Ordering::SeqCst);

            // unmask the relevant DAIF bits
            registers::DAIFClr.set(bits_to_clear as u64);
        }
    }
}
