// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2026 by the author(s)
//
// Author(s):
//   - Niu Zhihong <zhihong@nzhnb.com>

//! Interrupt Mask Bits [clear]
//!
//! Allows atomically clearing (unmasking) individual DAIF interrupt mask bits
//! without affecting other bits, using the `MSR DAIFClr, #<imm>` instruction.
//!
//! This avoids the read-modify-write hazard of using `MSR DAIF, <Xt>` directly.

use tock_registers::{interfaces::Writeable, register_bitfields};

register_bitfields! {u64,
    pub DAIFClr [
        /// Process state D mask. The possible values of this bit are:
        ///
        /// 0 No effect on the D mask.
        ///
        /// 1 Watchpoint, Breakpoint, and Software Step exceptions targeted at
        ///   the current Exception level are unmasked.
        D OFFSET(3) NUMBITS(1) [
            NoEffect = 0,
            Unmask = 1
        ],

        /// SError interrupt mask bit. The possible values of this bit are:
        ///
        /// 0 No effect on the A mask.
        /// 1 Exception unmasked.
        A OFFSET(2) NUMBITS(1) [
            NoEffect = 0,
            Unmask = 1
        ],

        /// IRQ mask bit. The possible values of this bit are:
        ///
        /// 0 No effect on the I mask.
        /// 1 Exception unmasked.
        I OFFSET(1) NUMBITS(1) [
            NoEffect = 0,
            Unmask = 1
        ],

        /// FIQ mask bit. The possible values of this bit are:
        ///
        /// 0 No effect on the F mask.
        /// 1 Exception unmasked.
        F OFFSET(0) NUMBITS(1) [
            NoEffect = 0,
            Unmask = 1
        ]
    ]
}

pub struct Reg;

impl Writeable for Reg {
    type T = u64;
    type R = DAIFClr::Register;

    #[cfg(target_arch = "aarch64")]
    #[inline]
    fn set(&self, value: u64) {
        // MSR DAIFClr takes a 4-bit immediate operand, so we must match on all
        // possible values and emit the corresponding instruction with a constant
        // immediate.
        let imm = value & 0xF;
        match imm {
            0b0000 => {}
            0b0001 => unsafe { core::arch::asm!("msr daifclr, #0x1", options(nomem, nostack)) },
            0b0010 => unsafe { core::arch::asm!("msr daifclr, #0x2", options(nomem, nostack)) },
            0b0011 => unsafe { core::arch::asm!("msr daifclr, #0x3", options(nomem, nostack)) },
            0b0100 => unsafe { core::arch::asm!("msr daifclr, #0x4", options(nomem, nostack)) },
            0b0101 => unsafe { core::arch::asm!("msr daifclr, #0x5", options(nomem, nostack)) },
            0b0110 => unsafe { core::arch::asm!("msr daifclr, #0x6", options(nomem, nostack)) },
            0b0111 => unsafe { core::arch::asm!("msr daifclr, #0x7", options(nomem, nostack)) },
            0b1000 => unsafe { core::arch::asm!("msr daifclr, #0x8", options(nomem, nostack)) },
            0b1001 => unsafe { core::arch::asm!("msr daifclr, #0x9", options(nomem, nostack)) },
            0b1010 => unsafe { core::arch::asm!("msr daifclr, #0xa", options(nomem, nostack)) },
            0b1011 => unsafe { core::arch::asm!("msr daifclr, #0xb", options(nomem, nostack)) },
            0b1100 => unsafe { core::arch::asm!("msr daifclr, #0xc", options(nomem, nostack)) },
            0b1101 => unsafe { core::arch::asm!("msr daifclr, #0xd", options(nomem, nostack)) },
            0b1110 => unsafe { core::arch::asm!("msr daifclr, #0xe", options(nomem, nostack)) },
            0b1111 => unsafe { core::arch::asm!("msr daifclr, #0xf", options(nomem, nostack)) },
            _ => unreachable!(),
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    fn set(&self, _value: u64) {
        unimplemented!()
    }
}

#[allow(non_upper_case_globals)]
pub const DAIFClr: Reg = Reg {};
