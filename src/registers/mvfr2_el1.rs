// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! AArch32 Media and VFP Feature Register 2 - EL1

use tock_registers::interfaces::Readable;

pub struct Reg;

register_bitfields! {u64,
    pub MVFR2_EL1 [
        /// Support for miscellaneous VFP features
        FPMisc OFFSET(4) NUMBITS(4) [
            NotImplemented = 0b0000,
            FloatingPointSelection = 0b0001,
            FloatingPointSelectionConvToIntDirectRound = 0b0010,
            FloatingPointSelectionConvToIntDirectRoundAndRoundToInt = 0b0011,
            FloatingPointSelectionConvToIntDirectRoundAndRoundToIntAndMaxMinNum = 0b0100,
        ]

        /// Support for miscellaneous Advanced SIMD features.
        SIMDMisc OFFSET(0) NUMBITS(4) [
            NotImplemented = 0b0000,
            FloatingPointToIntWithDirectRounding = 0b0001,
            FloatingPointToIntWithDirectRoundingAndFloatingPointRoundToInt = 0b0010,
            FloatingPointToIntWithDirectRoundingAndFloatingPointRoundToIntAndMaxMinNum = 0b0011,
        ]
    ]
}

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "MVFR2_EL1", "x");
}

pub const MVFR2_EL1: Reg = Reg;
