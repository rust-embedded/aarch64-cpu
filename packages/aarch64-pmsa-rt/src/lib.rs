//! A startup library for AArch64 systems that implement a PMSA (Protected
//! Memory Systems Architecture), i.e. systems without a MMU
//!
//! This library does not assume that a bootloader is present and can be used at
//! any Exception Level
//!
//! # References
//!
//! - ARM-R64: Arm Architecture Reference Manual for R-profile AArch64 architecture. DDI 0628.

#![no_std]

use core::{
    arch::{asm, naked_asm},
    mem,
};

use aarch64_cpu::registers::{self, DAIF, Readable as _, Writeable as _};

mod sections;

#[doc(inline)]
pub use sections::Section;

/// Executes `f` at one lower Exception Level on the given `stack` memory
///
/// `f` will inherit the current Interrupt Mask Bits (DAIF)
pub fn drop_exception_level(f: extern "C" fn() -> !, stack: Stack) -> ! {
    /// This function is the first thing that runs at the lower EL and serves
    /// as a "trampoline" into the user-defined entry point `f`. It does the
    /// following:
    ///
    /// 1. sets the Stack Pointer of the _lower_ Exception Level to `initial_sp`, which is passed in
    ///    register X1;
    ///
    /// 2. registers the vector table, configured via `exception_handlers`, at the _lower_ Exception
    ///    Level so that it's not left unitialized; and
    ///
    /// 3. finally calls into the user provide entry point `f` (parent function's first argument),
    ///    which is passed in register X0
    // the assembly used here is based off the following Rust code compiled with
    // rustc v1.93.1 and opt-level='z'
    // FIXME: GDB's `backtrace` cannot see the higher EL stack frames
    // ```
    //
    // extern "C" fn _start(x0: usize, x1: usize) -> ! {
    //     unsafe extern "C" {
    //         fn set_vbar();
    //     }
    //     unsafe {
    //         asm!("mov SP, {}", in(reg) x1);
    //         set_vbar();
    //         asm!("br {}", in(reg) x0, options(noreturn))
    //     }
    // }
    // ```
    #[unsafe(naked)]
    extern "C" fn lower_el_entry(f: usize, initial_sp: usize) -> ! {
        naked_asm!(
            // 1. set stack pointer for lower EL
            "mov SP, x1",
            // push link register (X30) and also X19 to keep the
            // stack 16-byte aligned
            "stp x30, x19, [sp, #-0x10]!",
            // stash X0, in X19, as `set_vbar` is allowed to use it without
            // restoring it
            "mov x19, x0",
            // 2. set vector table for lower EL
            "bl {set_vbar}",
            // 3. call into user provided entry point
            "br x19",
            set_vbar = sym set_vbar,
        );
    }

    let el = registers::CurrentEL.read(registers::CurrentEL::EL);

    match el {
        0 => panic!("already at EL0"),
        1 => todo!(),
        2 => {
            let daif = DAIF.get();
            // [ARM-R64/C5.2.15] EL1 with SP_EL1 (EL1h)
            let m = 0b0101;
            registers::SPSR_EL2.set(daif | m);

            registers::ELR_EL2.set(lower_el_entry as *const () as u64);
        }
        3 => todo!(),
        _ => unreachable!(),
    }

    // SAFETY: SPSR and ELR have been set to sensible values
    unsafe {
        asm!(
            "eret",
            in("x0") f,
            // the stack grows downwards ("full descending stack") so the
            // initial SP value is the higher boundary of the memory block
            in("x1") stack.higher(),
            options(noreturn),
        )
    }
}

/// Start-up code for AArch64 - supports both Armv8-A and Armv8-R, at EL1 or EL2
///
/// This is our default start-up code. You may instead supply your own `_start` function, which may
/// call this function if required.
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub unsafe extern "C" fn _default_start() -> ! {
    core::arch::naked_asm!(
        r#"
            mrs     x0, CPACR_EL1       // disable trapping on FPU/SIMD instructions in EL1
            orr     x0, x0, #(0b11 << 20)
            msr     CPACR_EL1, x0
            isb
            bl      {get_cpuid}         // Put CPUID in x0
            cbz     x0, core0_only      // Check if CPUID is zero
        loop_wfi:
            dsb     SY                  // Clear all pending data accesses
            wfi                         // Go to sleep
            b       loop_wfi            // Loop if we wake up
        core0_only:
            ldr     x0, =__sbss         // zero .bss section which is marked NOLOAD in the linker script
            ldr     x1, =__ebss
        bss_loop:                       // zeroing loop
            cmp     x0, x1
            b.hs    bss_done
            str     xzr, [x0], #0x8
            b       bss_loop
        bss_done:
            ldr     x0, =__eboot_stack  // set up stack pointer
            mov     sp, x0
            b {rust_start}              // call Rust part of the startup
        "#,
        rust_start = sym rust_start,
        get_cpuid = sym get_cpuid,
    )
}

/// Get the CPUID for the running CPU.
///
/// This algorithm is as described by Arm in
/// [Learn the Architecture - Booting the Cortex-R82 Guide](https://developer.arm.com/documentation/109917/0002/EL2-boot-steps?lang=en)
#[unsafe(naked)]
pub extern "C" fn get_cpuid() -> u64 {
    core::arch::naked_asm!(
        r#"
        mrs x0, MIDR_EL1
        ubfm    x0, x0, #4, #12 // extract PartNum
        mrs x0, MPIDR_EL1
        ubfm    x1, x0, 0, 8    // Get MPIDR_EL1_AFF0
        ubfm    x2, x0, 8, 8    // Get MPIDR_EL1_AFF1
        add     x0, x1, x2, LSL #2
        ret
        "#,
    )
}

/// Entry point to the Rust world
///
/// Called by `_default_start`, or by your start-up routine.
///
/// Does some Rust based initialisation, then calls `_rust_main`.
extern "C" fn rust_start() -> ! {
    unsafe extern "Rust" {
        fn _rust_main() -> !;
    }

    set_vbar();

    // SAFETY: symbol provided by `entry!` macro which enforces the
    // signature specified in the `extern` block
    unsafe { _rust_main() }
}

/// Set up the Vector Base Address Register (VBAR) for the current EL
extern "C" fn set_vbar() {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);

    match el {
        0 => {
            // no exception can be taken to EL0 so nothing to do in this case
        }
        1 => {
            unsafe extern "C" {
                fn _vbar_el1();
            }
            registers::VBAR_EL1.set(_vbar_el1 as *const () as u64)
        }
        2 => {
            unsafe extern "C" {
                fn _vbar_el2();
            }
            registers::VBAR_EL2.set(_vbar_el2 as *const () as u64)
        }
        3 => {
            unsafe extern "C" {
                fn _vbar_el3();
            }
            registers::VBAR_EL3.set(_vbar_el3 as *const () as u64)
        }
        _ => unreachable!(),
    }
}

/// Registers pushed onto the stack upon entering an exception handler
#[derive(Debug)]
// we may want to add other registers as fields in the future
#[non_exhaustive]
#[repr(C)]
#[repr(align(16))]
pub struct StackedRegisters {
    /// X0-X18
    pub x: [u64; 19],
    /// X29 AKA Frame Pointer
    pub x29: usize,
    /// X30 AKA Procedure Link Register
    pub x30: u64,
    /// Exception Link Register
    pub elr: usize,
    /// Saved Program Status Register
    pub spsr: usize,
}

// NOTE this size must match stack space reserved in the prologue of the
// exception handlers
const _: () = assert!(8 * 24 == mem::size_of::<StackedRegisters>());

/// Registers exception handlers
///
/// Takes as argument a path to the struct that implements the
/// `ExceptionHandlers` trait
#[macro_export]
macro_rules! exception_handlers {
    ($handlers:path) => {
        core::arch::global_asm!("
.macro push_registers el:req
    // X19-X28 are callee-saved so any code called from here will push them
    // onto the stack in its prologue; everything else we need to push and
    // pop ourselves
    //
    // To support exception nesting we need to also push and pop ELR and
    // SPSR at the current EL as a nested exception will overwrite those
    // system registers
    //
    // `!` adds -8*22 to SP _before_ pushing x0, x1 on the stack
    stp x0,  x1,  [sp, #-(8 * 24)]!
    stp x2,  x3,  [sp, #8 * 2]
    stp x4,  x5,  [sp, #8 * 4]
    stp x6,  x7,  [sp, #8 * 6]
    stp x8,  x9,  [sp, #8 * 8]
    stp x10, x11, [sp, #8 * 10]
    stp x12, x13, [sp, #8 * 12]
    stp x14, x15, [sp, #8 * 14]
    stp x16, x17, [sp, #8 * 16]
    stp x18, x29, [sp, #8 * 18]
    mrs x0,  elr_\\el
    stp x30, x0,  [sp, #8 * 20]
    mrs x1,  spsr_\\el
    str x1,       [sp, #8 * 22]
.endm

.macro pop_registers el:req
    ldp x2,  x3,  [sp, #8 * 2]
    ldp x4,  x5,  [sp, #8 * 4]
    ldp x6,  x7,  [sp, #8 * 6]
    ldp x8,  x9,  [sp, #8 * 8]
    ldp x10, x11, [sp, #8 * 10]
    ldp x12, x13, [sp, #8 * 12]
    ldp x14, x15, [sp, #8 * 14]
    ldp x16, x17, [sp, #8 * 16]
    ldp x18, x29, [sp, #8 * 18]
    ldp x30, x0,  [sp, #8 * 20]
    msr elr_\\el, x0
    ldr x1,       [sp, #8 * 22]
    msr spsr_\\el, x1
    ldp x0,  x1,  [sp], #8 * 24
.endm

.macro exception_vectors el:req
    // [ARM-R64/D12.2.100] Vector Base Address needs to be 2048-byte aligned
    .balign 0x800
    .global _vbar_\\el
    .section .text._vbar_\\el
_vbar_\\el:
    // [ARM-R64/D1.3.1.6] describes the layout; each vector is 0x80 bytes in size
    // 0x000: 'Current Exception level with SP_EL0' -- these entries are unused
    nop
    .balign 0x200

    // 0x200: 'Current Exception level with SP_ELx, x > 0'
vbar_sync_current_\\el:
    push_registers \\el
    mov x0, sp
    bl {sync_current}
    pop_registers \\el
    eret
    .balign 0x80

vbar_irq_current_\\el:
    push_registers \\el
    mov x0, sp
    bl {irq_current}
    pop_registers \\el
    eret
    .balign 0x80

vbar_fiq_current_\\el:
    push_registers \\el
    mov x0, sp
    bl {fiq_current}
    pop_registers \\el
    eret
    .balign 0x80

vbar_serror_current_\\el:
    push_registers \\el
    mov x0, sp
    bl {serror_current}
    pop_registers \\el
    eret
    .balign 0x80

    // 0x400: 'Lower Exception level'
vbar_sync_lower_\\el:
    push_registers \\el
    mov x0, sp
    bl {sync_lower}
    pop_registers \\el
    eret
    .balign 0x80

vbar_irq_lower_\\el:
    push_registers \\el
    mov x0, sp
    bl {irq_lower}
    pop_registers \\el
    eret
    .balign 0x80

vbar_fiq_lower_\\el:
    push_registers \\el
    mov x0, sp
    bl {fiq_lower}
    pop_registers \\el
    eret
    .balign 0x80

vbar_serror_lower_\\el:
    push_registers \\el
    mov x0, sp
    bl {serror_lower}
    pop_registers \\el
    eret
    .balign 0x80
.endm

    exception_vectors el1
    exception_vectors el2
    exception_vectors el3
",
            sync_current = sym <$handlers as $crate::ExceptionHandlers>::sync_current,
            irq_current = sym <$handlers as $crate::ExceptionHandlers>::irq_current,
            fiq_current = sym <$handlers as $crate::ExceptionHandlers>::fiq_current,
            serror_current = sym <$handlers as $crate::ExceptionHandlers>::serror_current,
            sync_lower = sym <$handlers as $crate::ExceptionHandlers>::sync_lower,
            irq_lower = sym <$handlers as $crate::ExceptionHandlers>::irq_lower,
            fiq_lower = sym <$handlers as $crate::ExceptionHandlers>::fiq_lower,
            serror_lower = sym <$handlers as $crate::ExceptionHandlers>::serror_lower,
        );
    };
}

/// Interface to statically register exception handlers
pub trait ExceptionHandlers {
    /// Handles Synchronous exceptions taken at the current EL
    extern "C" fn sync_current(context: &StackedRegisters) {
        _ = context;
        panic!("unhandled Synchronous exception at current EL")
    }

    /// Handles IRQ exceptions taken at the current EL
    extern "C" fn irq_current(context: &StackedRegisters) {
        _ = context;
        panic!("unhandled IRQ exception at current EL")
    }

    /// Handles IFQ exceptions taken at the current EL
    extern "C" fn fiq_current(context: &StackedRegisters) {
        _ = context;
        panic!("unhandled FIQ exception at current EL")
    }

    /// Handles SError exceptions taken at the current EL
    extern "C" fn serror_current(context: &StackedRegisters) {
        _ = context;
        panic!("unhandled SError exception at current EL")
    }

    /// Handles Synchronous exceptions taken from the immediate lower EL
    extern "C" fn sync_lower(context: &StackedRegisters) {
        _ = context;
        panic!("unhandled Synchronous exception at lower EL")
    }

    /// Handles IRQ exceptions taken from the immediate lower EL
    extern "C" fn irq_lower(context: &StackedRegisters) {
        _ = context;
        panic!("unhandled IRQ exception at lower EL")
    }

    /// Handles FIQ exceptions taken from the immediate lower EL
    extern "C" fn fiq_lower(context: &StackedRegisters) {
        _ = context;
        panic!("unhandled FIQ exception at lower EL")
    }

    /// Handles SError exceptions taken from the immediate lower EL
    extern "C" fn serror_lower(context: &StackedRegisters) {
        _ = context;
        panic!("unhandled SError exception at lower EL")
    }
}

/// Marks the main function of the binary and reserves space for the boot stack
///
/// 4 KiB is reserved for the boot stack by default. A different size and
/// alignment may be configured by passing the size and alignment, in bytes, as
/// extra arguments to the macro:
///
/// For example, `entry!(main, stack_size=16*1024, stack_align=64);` reserves
/// 16 KiB of stack space and ensures it's 64-byte aligned
///
/// Note that due to requirements by the architecture the stack must be at least
/// 16-byte aligned.
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        entry!($path, stack_size = 4 * 1024, stack_align = 16);
    };

    ($path:path, stack_size=$size:expr) => {
        entry!($path, stack_size = $size, stack_align = 16);
    };

    ($path:path, stack_size=$size:expr, stack_align=$align:expr) => {
        // inside `const` to prevent adding `__entry_impl_detail` to the surrounding namespace
        const _: () = {
            // name starts with underscore to (hopefully) avoid a name collision with `$path`
            #[unsafe(export_name = "_rust_main")]
            fn _entry_impl_detail() -> ! {
                #[repr(C)]
                #[repr(align($align))]
                struct Aligned {
                    bytes: [u8; $size],
                }

                impl Aligned {
                    const fn new() -> Self {
                        assert!($align >= 16, "stack must be at least 16-byte aligned");
                        assert!($size > 0, "stack cannot be zero-sized");

                        Self { bytes: [0; $size] }
                    }
                }

                #[unsafe(no_mangle)] // ensures this is not GC-ed by the compiler
                #[unsafe(link_section = ".stack.boot_stack")]
                static mut _BOOT_STACK: Aligned = Aligned::new();

                // type check `$path`
                let _: fn() -> ! = $path;

                $path()
            }
        };
    };
}

/// Statically allocates `$size` bytes of stack memory
///
/// This returns an `Option` because it gives back an owning singleton so it
/// must not be called more than once.
///
/// The default alignment for stack memory is 16 bytes; this can be overwritten
/// using the optional second argument.
///
/// For example, `alloc_stack!(16 * 1024, align = 64)` aligns the
/// stack memory to a 64-byte boundary.
#[macro_export]
macro_rules! alloc_stack {
    ($size:expr) => {{ alloc_stack!($size, align = 16) }};

    ($size:expr, align=$align:expr) => {{
        static ONCE: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

        // `Relaxed` (no synchronization barrier) as this does not interact with
        // _other_ memory; it only hands out a new static memory allocation
        if ONCE.swap(true, core::sync::atomic::Ordering::Relaxed) {
            // already called
            None
        } else {
            #[repr(C)]
            #[repr(align($align))]
            struct Aligned {
                bytes: [u8; $size],
            }

            impl Aligned {
                const fn new() -> Self {
                    assert!($align >= 16, "stack alignment must be at least 16 bytes");
                    assert!($size > 0, "stack cannot be zero-sized");

                    Self { bytes: [0; $size] }
                }
            }

            #[unsafe(link_section = ".stack")]
            static mut STACK: Aligned = Aligned::new();

            let lower = (&raw mut STACK as usize);
            let size = core::mem::size_of::<Aligned>();

            // SAFETY: the atomic bool guard ensures that the
            // static mut variable is only used once
            Some(unsafe { $crate::Stack::new(lower, size) })
        }
    }};
}

/// An owning pointer into statically allocated, unused stack memory
///
/// Use the `alloc_stack!` macro to create this type
pub struct Stack {
    lower: usize,
    size: usize,
}

#[doc(hidden)]
impl Stack {
    /// # Safety
    /// As this is an owning pointer, no more than one `Stack` instance should
    /// point into the given `memory`
    pub unsafe fn new(lower: usize, size: usize) -> Self {
        Self { lower, size }
    }

    /// Returns the address of the lower boundary of the stack memory
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Returns the address of the higher boundary of the stack memory
    ///
    /// Note that the higher address points *outside* the stack memory
    pub fn higher(&self) -> usize {
        self.lower + self.size
    }
    /// Returns the size of this stack memory in bytes
    pub fn size(&self) -> usize {
        self.size
    }
}
