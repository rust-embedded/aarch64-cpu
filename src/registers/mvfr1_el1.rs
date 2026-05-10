// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! AArch32 Media and VFP Feature Register 1 - EL1

use tock_registers::interfaces::Readable;
use tock_registers::register_bitfields;

pub struct Reg;

register_bitfields! {u64,
    pub MVFR1_EL1 [
        /// Advanced SIMD Fused Multiply-Accumulate
        SIMDFMAC OFFSET(4) NUMBITS(28) [
            NotImplemented = 0b0000,
            Implemnted = 0b0001,
        ],

        /// Floating-Point Half-Precision
        FPHP OFFSET(4) NUMBITS(24) [
            NotSupported = 0b0000,
            SupportedSingleHalfPrecisionConversion = 0b0001,
            SupportedSingleHalfDoublePrecisionConversion = 0b0010,
            SupportedSingleHalfDoublePrecisionConversionAndArithmetic = 0b0011,
        ],

        /// Advanced SIMD Half-Precision
        SIMDHP OFFSET(4) NUMBITS(20) [
            NotSupported = 0b0000,
            SupportedForConversion = 0b0001,
            SupportedForConversionAndArithmetic = 0b0010,
        ],

        /// Advanced SIMD Single-Precision
        SIMDSP OFFSET(4) NUMBITS(16) [
            NotImplemented = 0b0000,
            Implemnted = 0b0001,
        ],

        /// Advanced SIMD Integer
        SIMDInt OFFSET(4) NUMBITS(12) [
            NotImplemented = 0b0000,
            Implemnted = 0b0001,
        ],

        /// Advanced SIMD Load/Store
        SIMDLS OFFSET(8) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemnted = 0b0001,
        ],

        /// Default NaN mode
        FPDNaN OFFSET(4) NUMBITS(4) [
            NotImplemented = 0b0000,
            Supported = 0b0001,
        ],

        /// Flush to Zero mode
        FPFtZ OFFSET(0) NUMBITS(4) [
            NotImplemented = 0b0000,
            Supported = 0b0001,
        ],
    ]
}

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "MVFR1_EL1", "x");
}

pub const MVFR1_EL1: Reg = Reg;
