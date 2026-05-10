// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! AArch64 Instruction Set Attribute Register 3- EL1

use tock_registers::interfaces::Readable;
use tock_registers::register_bitfields;

pub struct Reg;

register_bitfields! {u64,
    pub ID_AA64ISAR3_EL1 [
        /// Support for `LDAP`, `LDAPP` and `STLP` instructions
        LSCP OFFSET(44) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for `SHUH` and `STCPH`
        LSCSHINT OFFSET(40) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for `DC ZGBVA` and `DC GBVA``
        MTETC OFFSET(36) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Indicates support for independant control of EL0 PAC and disabling BTI landing pad on PAC instructions
        PAC_frac2 OFFSET(32) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Indicates support for some floating-point and integer conversion instructions
        FPRCVT OFFSET(28) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for some unprivileged load and store instructions
        LSUI OFFSET(24) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Indicates support for `DC CIVAOC` and `DC CVAOC`
        OCCMO OFFSET(20) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Indicates support for some atomic floating-point in-memory instructions
        LSFE OFFSET(16) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Indicates the implementation of PSTATE.PACM
        PACM OFFSET(12) NUMBITS(4) [
            NotSupported = 0b0000,
            Trivial = 0b0001,
            Full = 0b0010,
        ],

        /// Support for TLBI VMALL for Dirty state
        TLBIW OFFSET(8) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Support for SIMD instructions that compute maximum and minimum absolute values
        FAMINMAX OFFSET(4) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for Checked Pointer Arithmetic instructions
        CPA OFFSET(0) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
            ImplementedCanBeEnabled = 0b0010,
        ],
    ]
}

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "ID_AA64ISAR3_EL1", "x");
}

pub const ID_AA64ISAR3_EL1: Reg = Reg;
