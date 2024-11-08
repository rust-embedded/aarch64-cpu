#[repr(transparent)]
pub struct IRQ(u64);

impl IRQ {
    pub fn new() -> Self {
        IRQ(0)
    }
    #[inline(always)]
    unsafe fn enable(&self) {
        unsafe { core::arch::asm!("msr daifclr, #2", options(nostack)) }
    }

    #[inline(always)]
    unsafe fn disable(&self) {
        unsafe { core::arch::asm!("msr daifclr, #2", options(nostack)) }
    }

    #[inline(always)]
    pub fn save_and_disable(&mut self) {
        unsafe {
            core::arch::asm!("mrs {}, daif", out(reg) self.0, options(nostack));
            self.disable();
        }
    }

    #[inline(always)]
    pub fn restore_and_enable(&self) {
        unsafe {
            core::arch::asm!("msr daif, {}", in(reg) self.0, options(nostack));
            self.enable();
        }
    }
}
