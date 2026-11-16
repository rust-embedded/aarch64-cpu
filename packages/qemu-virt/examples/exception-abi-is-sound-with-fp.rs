//! Checks that the ABI of the exception handler is sound
// runner: qemu-system-aarch64 -cpu neoverse-v1 -machine virt,virtualization=on -nographic -semihosting -kernel
// rustflags: -C force-frame-pointers=true
// ignore: aarch64-unknown-none-softfloat

#![no_std]
#![no_main]

use core::arch::aarch64::{vdupq_n_f64, vgetq_lane_f64, vreinterpretq_p128_f64};
use core::arch::asm;

use aarch64_cpu::registers::{self, Read as _};
use aarch64_pmsa_rt::{ExceptionHandlers, StackedRegisters, entry, exception_handlers};
use semihosting::{eprintln, println, process};

entry!(main, stack_size = 16 * 1024);

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
        mut q0,
        mut q1,
        mut q2,
        mut q3,
        mut q4,
        mut q5,
        mut q6,
        mut q7,
        mut q8,
        mut q9,
        mut q10,
        mut q11,
        mut q12,
        mut q13,
        mut q14,
        mut q15,
        mut q16,
        mut q17,
        mut q18,
        mut q19,
        mut q20,
        mut q21,
        mut q22,
        mut q23,
        mut q24,
        mut q25,
        mut q26,
        mut q27,
        mut q28,
        mut q29,
        mut q30,
        mut q31,
    ) = unsafe {
        (
            0,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            11,
            12,
            13,
            14,
            15,
            16,
            17,
            18,
            30,
            // we cannot write 128-bit SIMD float constants, but we can make a
            // 128 bit SIMD value by duplicating a literal f64 value
            vdupq_n_f64(0.0),
            vdupq_n_f64(1.0),
            vdupq_n_f64(2.0),
            vdupq_n_f64(3.0),
            vdupq_n_f64(4.0),
            vdupq_n_f64(5.0),
            vdupq_n_f64(6.0),
            vdupq_n_f64(7.0),
            vdupq_n_f64(8.0),
            vdupq_n_f64(9.0),
            vdupq_n_f64(10.0),
            vdupq_n_f64(11.0),
            vdupq_n_f64(12.0),
            vdupq_n_f64(13.0),
            vdupq_n_f64(14.0),
            vdupq_n_f64(15.0),
            vdupq_n_f64(16.0),
            vdupq_n_f64(17.0),
            vdupq_n_f64(18.0),
            vdupq_n_f64(19.0),
            vdupq_n_f64(20.0),
            vdupq_n_f64(21.0),
            vdupq_n_f64(22.0),
            vdupq_n_f64(23.0),
            vdupq_n_f64(24.0),
            vdupq_n_f64(25.0),
            vdupq_n_f64(26.0),
            vdupq_n_f64(27.0),
            vdupq_n_f64(28.0),
            vdupq_n_f64(29.0),
            vdupq_n_f64(30.0),
            vdupq_n_f64(31.0),
        )
    };

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
            inout("q0") q0,
            inout("q1") q1,
            inout("q2") q2,
            inout("q3") q3,
            inout("q4") q4,
            inout("q5") q5,
            inout("q6") q6,
            inout("q7") q7,
            inout("q8") q8,
            inout("q9") q9,
            inout("q10") q10,
            inout("q11") q11,
            inout("q12") q12,
            inout("q13") q13,
            inout("q14") q14,
            inout("q15") q15,
            inout("q16") q16,
            inout("q17") q17,
            inout("q18") q18,
            inout("q19") q19,
            inout("q20") q20,
            inout("q21") q21,
            inout("q22") q22,
            inout("q23") q23,
            inout("q24") q24,
            inout("q25") q25,
            inout("q26") q26,
            inout("q27") q27,
            inout("q28") q28,
            inout("q29") q29,
            inout("q30") q30,
            inout("q31") q31,
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
    // get each half of the 2xf64 SIMD value at a time
    assert_eq!(0.0, unsafe { vgetq_lane_f64(q0, 0) });
    assert_eq!(0.0, unsafe { vgetq_lane_f64(q0, 1) });
    assert_eq!(1.0, unsafe { vgetq_lane_f64(q1, 0) });
    assert_eq!(1.0, unsafe { vgetq_lane_f64(q1, 1) });
    assert_eq!(2.0, unsafe { vgetq_lane_f64(q2, 0) });
    assert_eq!(2.0, unsafe { vgetq_lane_f64(q2, 1) });
    assert_eq!(3.0, unsafe { vgetq_lane_f64(q3, 0) });
    assert_eq!(3.0, unsafe { vgetq_lane_f64(q3, 1) });
    assert_eq!(4.0, unsafe { vgetq_lane_f64(q4, 0) });
    assert_eq!(4.0, unsafe { vgetq_lane_f64(q4, 1) });
    assert_eq!(5.0, unsafe { vgetq_lane_f64(q5, 0) });
    assert_eq!(5.0, unsafe { vgetq_lane_f64(q5, 1) });
    assert_eq!(6.0, unsafe { vgetq_lane_f64(q6, 0) });
    assert_eq!(6.0, unsafe { vgetq_lane_f64(q6, 1) });
    assert_eq!(7.0, unsafe { vgetq_lane_f64(q7, 0) });
    assert_eq!(7.0, unsafe { vgetq_lane_f64(q7, 1) });
    assert_eq!(8.0, unsafe { vgetq_lane_f64(q8, 0) });
    assert_eq!(8.0, unsafe { vgetq_lane_f64(q8, 1) });
    assert_eq!(9.0, unsafe { vgetq_lane_f64(q9, 0) });
    assert_eq!(9.0, unsafe { vgetq_lane_f64(q9, 1) });
    assert_eq!(10.0, unsafe { vgetq_lane_f64(q10, 0) });
    assert_eq!(10.0, unsafe { vgetq_lane_f64(q10, 1) });
    assert_eq!(11.0, unsafe { vgetq_lane_f64(q11, 0) });
    assert_eq!(11.0, unsafe { vgetq_lane_f64(q11, 1) });
    assert_eq!(12.0, unsafe { vgetq_lane_f64(q12, 0) });
    assert_eq!(12.0, unsafe { vgetq_lane_f64(q12, 1) });
    assert_eq!(13.0, unsafe { vgetq_lane_f64(q13, 0) });
    assert_eq!(13.0, unsafe { vgetq_lane_f64(q13, 1) });
    assert_eq!(14.0, unsafe { vgetq_lane_f64(q14, 0) });
    assert_eq!(14.0, unsafe { vgetq_lane_f64(q14, 1) });
    assert_eq!(15.0, unsafe { vgetq_lane_f64(q15, 0) });
    assert_eq!(15.0, unsafe { vgetq_lane_f64(q15, 1) });
    assert_eq!(16.0, unsafe { vgetq_lane_f64(q16, 0) });
    assert_eq!(16.0, unsafe { vgetq_lane_f64(q16, 1) });
    assert_eq!(17.0, unsafe { vgetq_lane_f64(q17, 0) });
    assert_eq!(17.0, unsafe { vgetq_lane_f64(q17, 1) });
    assert_eq!(18.0, unsafe { vgetq_lane_f64(q18, 0) });
    assert_eq!(18.0, unsafe { vgetq_lane_f64(q18, 1) });
    assert_eq!(19.0, unsafe { vgetq_lane_f64(q19, 0) });
    assert_eq!(19.0, unsafe { vgetq_lane_f64(q19, 1) });
    assert_eq!(20.0, unsafe { vgetq_lane_f64(q20, 0) });
    assert_eq!(20.0, unsafe { vgetq_lane_f64(q20, 1) });
    assert_eq!(21.0, unsafe { vgetq_lane_f64(q21, 0) });
    assert_eq!(21.0, unsafe { vgetq_lane_f64(q21, 1) });
    assert_eq!(22.0, unsafe { vgetq_lane_f64(q22, 0) });
    assert_eq!(22.0, unsafe { vgetq_lane_f64(q22, 1) });
    assert_eq!(23.0, unsafe { vgetq_lane_f64(q23, 0) });
    assert_eq!(23.0, unsafe { vgetq_lane_f64(q23, 1) });
    assert_eq!(24.0, unsafe { vgetq_lane_f64(q24, 0) });
    assert_eq!(24.0, unsafe { vgetq_lane_f64(q24, 1) });
    assert_eq!(25.0, unsafe { vgetq_lane_f64(q25, 0) });
    assert_eq!(25.0, unsafe { vgetq_lane_f64(q25, 1) });
    assert_eq!(26.0, unsafe { vgetq_lane_f64(q26, 0) });
    assert_eq!(26.0, unsafe { vgetq_lane_f64(q26, 1) });
    assert_eq!(27.0, unsafe { vgetq_lane_f64(q27, 0) });
    assert_eq!(27.0, unsafe { vgetq_lane_f64(q27, 1) });
    assert_eq!(28.0, unsafe { vgetq_lane_f64(q28, 0) });
    assert_eq!(28.0, unsafe { vgetq_lane_f64(q28, 1) });
    assert_eq!(29.0, unsafe { vgetq_lane_f64(q29, 0) });
    assert_eq!(29.0, unsafe { vgetq_lane_f64(q29, 1) });
    assert_eq!(30.0, unsafe { vgetq_lane_f64(q30, 0) });
    assert_eq!(30.0, unsafe { vgetq_lane_f64(q30, 1) });
    assert_eq!(31.0, unsafe { vgetq_lane_f64(q31, 0) });
    assert_eq!(31.0, unsafe { vgetq_lane_f64(q31, 1) });

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
            x,
            x29,
            x30,
            elr,
            q,
            ..
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
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(0.0)) }, q[0]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(1.0)) }, q[1]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(2.0)) }, q[2]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(3.0)) }, q[3]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(4.0)) }, q[4]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(5.0)) }, q[5]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(6.0)) }, q[6]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(7.0)) }, q[7]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(8.0)) }, q[8]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(9.0)) }, q[9]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(10.0)) }, q[10]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(11.0)) }, q[11]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(12.0)) }, q[12]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(13.0)) }, q[13]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(14.0)) }, q[14]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(15.0)) }, q[15]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(16.0)) }, q[16]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(17.0)) }, q[17]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(18.0)) }, q[18]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(19.0)) }, q[19]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(20.0)) }, q[20]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(21.0)) }, q[21]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(22.0)) }, q[22]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(23.0)) }, q[23]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(24.0)) }, q[24]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(25.0)) }, q[25]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(26.0)) }, q[26]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(27.0)) }, q[27]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(28.0)) }, q[28]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(29.0)) }, q[29]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(30.0)) }, q[30]);
        assert_eq!(unsafe { vreinterpretq_p128_f64(vdupq_n_f64(31.0)) }, q[31]);

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
                // writing to dX will trash both qX and sX, as they overlap
                in("d0") 999.0,
                in("d1") 999.0,
                in("d2") 999.0,
                in("d3") 999.0,
                in("d4") 999.0,
                in("d5") 999.0,
                in("d6") 999.0,
                in("d7") 999.0,
                in("d8") 999.0,
                in("d9") 999.0,
                in("d10") 999.0,
                in("d11") 999.0,
                in("d12") 999.0,
                in("d13") 999.0,
                in("d14") 999.0,
                in("d15") 999.0,
                in("d16") 999.0,
                in("d17") 999.0,
                in("d18") 999.0,
                in("d19") 999.0,
                in("d20") 999.0,
                in("d21") 999.0,
                in("d22") 999.0,
                in("d23") 999.0,
                in("d24") 999.0,
                in("d25") 999.0,
                in("d26") 999.0,
                in("d27") 999.0,
                in("d28") 999.0,
                in("d29") 999.0,
                in("d30") 999.0,
                in("d31") 999.0,
            )
        }
    }
}
