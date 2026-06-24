//! Checks that the ABI of the exception handler is sound
// runner: qemu-system-aarch64 -cpu neoverse-v1 -machine virt,virtualization=on -nographic -semihosting -kernel
// rustflags: -C force-frame-pointers=true

#![no_std]
#![no_main]

use core::arch::asm;

use aarch64_cpu::registers::{self, Readable as _};
use aarch64_pmsa_rt::{ExceptionHandlers, StackedRegisters, entry, exception_handlers};
use semihosting::{eprintln, println, process};

entry!(main);

fn main() -> ! {
    let el = registers::CurrentEL.read(registers::CurrentEL::EL);
    println!("running at EL{el}");
    assert_eq!(2, el, "this example must run at EL2");

    let (
        mut x0,
        mut x1,
        mut x2,
        mut x3,
        mut x4,
        mut x5,
        mut x6,
        mut x7,
        mut x8,
        mut x9,
        mut x10,
        mut x11,
        mut x12,
        mut x13,
        mut x14,
        mut x15,
        mut x16,
        mut x17,
        mut x18,
        mut x30,
    ) = (
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 30,
    );
    // SAFETY: VBAR_EL2 has been set
    unsafe {
        asm!(
            "SVC 0x0",
            inout("x0") x0,
            inout("x1") x1,
            inout("x2") x2,
            inout("x3") x3,
            inout("x4") x4,
            inout("x5") x5,
            inout("x6") x6,
            inout("x7") x7,
            inout("x8") x8,
            inout("x9") x9,
            inout("x10") x10,
            inout("x11") x11,
            inout("x12") x12,
            inout("x13") x13,
            inout("x14") x14,
            inout("x15") x15,
            inout("x16") x16,
            inout("x17") x17,
            inout("x18") x18,
            inout("x30") x30,
        )
    }

    // context is correctly restored
    assert_eq!(0, x0);
    assert_eq!(1, x1);
    assert_eq!(2, x2);
    assert_eq!(3, x3);
    assert_eq!(4, x4);
    assert_eq!(5, x5);
    assert_eq!(6, x6);
    assert_eq!(7, x7);
    assert_eq!(8, x8);
    assert_eq!(9, x9);
    assert_eq!(10, x10);
    assert_eq!(11, x11);
    assert_eq!(12, x12);
    assert_eq!(13, x13);
    assert_eq!(14, x14);
    assert_eq!(15, x15);
    assert_eq!(16, x16);
    assert_eq!(17, x17);
    assert_eq!(18, x18);
    assert_eq!(30, x30);

    process::exit(0)
}

exception_handlers!(Handlers);
struct Handlers;
impl ExceptionHandlers for Handlers {
    extern "C" fn sync_current(context: &StackedRegisters) {
        const RAM_SIZE: usize = 128 * 1024 * 1024;
        const RAM_START: usize = 0x4000_0000;
        const ARM_INSN_SIZE: usize = 4;
        // machine encoded `SVC 0x00` instruction
        const SVC_INSN: u32 = 0xd400_0001;

        let StackedRegisters {
            x, x29, x30, elr, ..
        } = *context;

        println!("handling SVC");

        // context is correctly captured
        assert_eq!(0, x[0]);
        assert_eq!(1, x[1]);
        assert_eq!(2, x[2]);
        assert_eq!(3, x[3]);
        assert_eq!(4, x[4]);
        assert_eq!(5, x[5]);
        assert_eq!(6, x[6]);
        assert_eq!(7, x[7]);
        assert_eq!(8, x[8]);
        assert_eq!(9, x[9]);
        assert_eq!(10, x[10]);
        assert_eq!(11, x[11]);
        assert_eq!(12, x[12]);
        assert_eq!(13, x[13]);
        assert_eq!(14, x[14]);
        assert_eq!(15, x[15]);
        assert_eq!(16, x[16]);
        assert_eq!(17, x[17]);
        assert_eq!(18, x[18]);
        assert_eq!(30, x30);

        // the code has been compiled with `-Cforce-frame-pointers=true`
        // the frame pointer is a snapshot of the stack pointer so it should
        // point into the RAM region
        let fp = x29;
        eprintln!("(FP={fp:#010x})");
        let ram_region = RAM_START..=(RAM_START + RAM_SIZE);
        assert!(ram_region.contains(&fp));

        // ELR points into the instruction that the exception will return to
        // as this is a synchronous exception the preceding instruction is
        // the exception trigger
        // SAFETY: ELR is a sample of the Program Counter and points into
        // `.text` which is defined/initialized
        let trigger = unsafe { ((elr - ARM_INSN_SIZE) as *const u32).read() };
        assert_eq!(SVC_INSN, trigger);

        // overwrite all registers that are preserved by the exception handler's
        // prologue/epilogue
        // SAFETY: no side effects
        unsafe {
            asm!(
                "nop",
                in("x0") 0xff,
                in("x1") 0xff,
                in("x2") 0xff,
                in("x3") 0xff,
                in("x4") 0xff,
                in("x5") 0xff,
                in("x6") 0xff,
                in("x7") 0xff,
                in("x8") 0xff,
                in("x9") 0xff,
                in("x10") 0xff,
                in("x11") 0xff,
                in("x12") 0xff,
                in("x13") 0xff,
                in("x14") 0xff,
                in("x15") 0xff,
                in("x16") 0xff,
                in("x17") 0xff,
                in("x18") 0xff,
                in("x30") 0xff,
            )
        }
    }
}
