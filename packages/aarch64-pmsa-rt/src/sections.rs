//! Code to get the output sections from the linker script
//!
//! This is useful if you want to set up the MPU

/// Represents one of the output sections in the linker script
#[derive(Debug, Copy, Clone)]
pub enum Section {
    /// The .text section
    ///
    /// Contains the executable code
    Text,
    /// The .rodata section
    ///
    /// Contains read-only static data
    Rodata,
    /// The .bss section
    ///
    /// Contains zero-initialised static data
    Bss,
    /// The .data section
    ///
    /// Contains non-zero-initialised static data
    Data,
    /// The .uninit section
    ///
    /// Contains non-initialised static data
    Uninit,
    /// The .stack section
    ///
    /// Contains the boot stack, and any additional stacks the code has set up (e.g. for EL1).
    Stack,
}

impl core::fmt::Display for Section {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.pad(match self {
            Section::Text => ".text",
            Section::Rodata => ".rodata",
            Section::Bss => ".bss",
            Section::Data => ".data",
            Section::Uninit => ".uninit",
            Section::Stack => ".stack",
        })
    }
}

impl Section {
    /// Create an iterator over all the regions
    pub fn iter() -> impl Iterator<Item = Section> {
        [
            Section::Text,
            Section::Rodata,
            Section::Bss,
            Section::Data,
            Section::Uninit,
            Section::Stack,
        ]
        .iter()
        .cloned()
    }

    /// Get the highest address of this region
    ///
    /// Technically, it gets the address *after* the region, suitable for a non-inclusive
    /// [Range<*const u8>](core::ops::Range).
    pub fn top(&self) -> *const u8 {
        use core::ptr::addr_of;
        unsafe extern "C" {
            static __etext: u8;
            static __erodata: u8;
            static __ebss: u8;
            static __edata: u8;
            static __euninit: u8;
            static __estack: u8;
        }
        match self {
            Section::Text => addr_of!(__etext),
            Section::Rodata => addr_of!(__erodata),
            Section::Bss => addr_of!(__ebss),
            Section::Data => addr_of!(__edata),
            Section::Uninit => addr_of!(__euninit),
            Section::Stack => addr_of!(__estack),
        }
    }

    /// Get the lowest address of this region
    pub fn bottom(&self) -> *const u8 {
        use core::ptr::addr_of;
        unsafe extern "C" {
            static __stext: u8;
            static __srodata: u8;
            static __sbss: u8;
            static __sdata: u8;
            static __suninit: u8;
            static __sstack: u8;
        }
        match self {
            Section::Text => addr_of!(__stext),
            Section::Rodata => addr_of!(__srodata),
            Section::Bss => addr_of!(__sbss),
            Section::Data => addr_of!(__sdata),
            Section::Uninit => addr_of!(__suninit),
            Section::Stack => addr_of!(__sstack),
        }
    }

    /// Get the range of this region.
    pub fn range(&self) -> Option<core::ops::Range<*const u8>> {
        let bottom = self.bottom();
        let top = self.top();
        if bottom != top {
            Some(bottom..top)
        } else {
            None
        }
    }

    /// Get the inclusive range of this region.
    ///
    /// This is the range you need to give to the PMSAv8 MPU code.
    pub fn mpu_range(&self) -> Option<core::ops::RangeInclusive<*const u8>> {
        let bottom = self.bottom();
        let top = self.top();
        let top_under = unsafe { top.offset(-1) };
        if bottom != top {
            Some(bottom..=top_under)
        } else {
            None
        }
    }

    /// Get the size of this region, in bytes
    pub fn size(&self) -> usize {
        self.top() as usize - self.bottom() as usize
    }
}
