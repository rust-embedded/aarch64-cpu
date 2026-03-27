// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! AArch32 Media and VFP Feature Register 0 - EL1

use tock_registers::interfaces::Readable;

pub struct Reg;

register_bitfields! {u64,
    pub MVFR0_EL1 [
        /// Floating-Point Rounding modes
        FPRound OFFSET(4) NUMBITS(28) [
            NotImplemented = 0b0000,
            Supported = 0b0001,
        ]

        /// Short Vectors
        FPShVVec OFFSET(4) NUMBITS(24) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ]

        /// Support for VFP square root
        FPSqrt OFFSET(4) NUMBITS(20) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ]

        /// Support for VFP divide
        FPDivide OFFSET(4) NUMBITS(16) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ]

        /// Floating-Point Exception Trapping
        FPTrap OFFSET(4) NUMBITS(12) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ]

        /// Floating-Point Double-Precision
        FPDP OFFSET(8) NUMBITS(4) [
            NotSupported = 0b0000,
            SupportedVFPv2 = 0b0001,
            SupportedVFPv3or4orArmv8 = 0b0010,
        ]

        /// Floating-Point Single-Precision
        FPSP OFFSET(4) NUMBITS(4) [
            NotSupported = 0b0000,
            SupportedVFPv2 = 0b0001,
            SupportedVFPv3or4 = 0b0010,
        ]

        /// Advanced SIMD registers
        SIMDReg OFFSET(0) NUMBITS(4) [
            NoSupport = 0b0000,
            FloatingPointSupport = 0b0001,
            AdvancedSIMDAndFloatingPoint = 0b0010,
        ]
    ]
}

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "MVFR0_EL1", "x");
}

pub const MVFR0_EL1: Reg = Reg;
