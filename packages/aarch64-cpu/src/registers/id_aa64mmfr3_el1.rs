// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! AArch64 Memory Model Feature Register 3 - EL1

use tock_registers::interfaces::Readable;
use tock_registers::register_bitfields;

pub struct Reg;

register_bitfields! {u64,
    pub ID_AA64MMFR3_EL1 [
        /// Speculative behaviour in a PAC authentication failure
        Spec_FPACC OFFSET(60) NUMBITS(4) [
            Undisclosed = 0b0000,
            NoDifference = 0b0001,
        ],

        /// Asynchronous Device error exceptions
        ADERR OFFSET(56) NUMBITS(4) [
            DependsOnRASv2 = 0b0000,
            AllExternal = 0b0001,
            DependsOnANERRDependant = 0b0010,
            DependsOnANERRIndependant = 0b0011,
        ],

        /// Synchronous Device error exceptions
        SDERR OFFSET(52) NUMBITS(4) [
            DependsOnRASv2 = 0b0000,
            AllExternal = 0b0001,
            DependsOnANERRDependant = 0b0010,
            DependsOnANERRIndependant = 0b0011,
        ],

        /// Asynchronous Normal error exceptions
        ANERR OFFSET(44) NUMBITS(4) [
            DependsOnRASv2 = 0b0000,
            AllExternal = 0b0001,
            DependsOnANERRDependant = 0b0010,
            DependsOnANERRIndependant = 0b0011,
        ],

        /// Synchronous Normal error exceptions
        SNERR OFFSET(40) NUMBITS(4) [
            DependsOnRASv2 = 0b0000,
            AllExternal = 0b0001,
            DependsOnANERRDependant = 0b0010,
            DependsOnANERRIndependant = 0b0011,
        ],

        /// 126-bit translation table descriptor support at stage 2
        D128_2 OFFSET(36) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// 126-bit translation table descriptor support
        D128 OFFSET(32) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Support for Memory Encryption Contexts
        MEC OFFSET(28) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Attribute Indexing
        AIE OFFSET(24) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Stage 2 Permission Overlay
        S2POE OFFSET(20) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Stage 1 Permission Overlay
        S1POE OFFSET(16) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
            R_2 = 0b0010,
        ],

        /// Stage 2 Permission Indirection
        S2PIE OFFSET(12) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Stage 1 Permission Indirection
        S1PIE OFFSET(8) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// SCTLR Extension
        SCTLRX OFFSET(4) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// TCR Extension
        TCRX OFFSET(0) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],
    ]
}

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "ID_AA64MMFR3_EL1", "x");
}

pub const ID_AA64MMFR3_EL1: Reg = Reg;
