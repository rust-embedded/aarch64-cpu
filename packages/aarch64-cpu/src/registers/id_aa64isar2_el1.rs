// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! AArch64 Instruction Set Attribute Register 2 - EL1

use tock_registers::interfaces::Readable;
use tock_registers::register_bitfields;

pub struct Reg;

register_bitfields! {u64,
    pub ID_AA64ISAR2_EL1 [
        /// Indicates support for address translation instructions that perform stage 1 translation without checking stage 1 permissions
        ATS1A OFFSET(60) NUMBITS(4) [
            NotImplmented = 0b0000,
            Implmented = 0b0001,
        ],

        /// Indicates support for advanced SIMD and SVE2 lookup instruction
        LUT OFFSET(56) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for common short sequence compression instructions
        CSSC OFFSET(52) NUMBITS(4) [
            NotImplemented = 0b0000,
            MinMaxAbsCnt = 0b0001,
            MinMaxAbsCntCB = 0b0010,
        ],

        /// RPRFM hint instruction
        RPRFM OFFSET(48) NUMBITS(4) [
            NotImplmented = 0b0000,
            Implmented = 0b0001,
        ],

        /// Indicates support for producer-consumer data placement hints
        PCDPHINT OFFSET(44) NUMBITS(4) [
            NotImplmented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Indicates whether PRFM instruction support a system level cache option
        PRFMSLC OFFSET(40) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Indicates support for system instructions that can take 128-bit inputs
        SYSINSTR_128 OFFSET(36) NUMBITS(4) [
            NotImplmented = 0b0000,
            Implmented = 0b0001,
        ],

        /// Indicates support for instructions to access 128-bit system registers
        SYSREG_128 OFFSET(32) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Indicates support for the CLRBHB instruction
        CLRBHB OFFSET(28) NUMBITS(4) [
            NotImplmented = 0b0000,
            Implmented = 0b0001,
        ],

        /// Indicates which bit is used to determine the size of the PAC field
        PAC_frac OFFSET(24) NUMBITS(4) [
            DependsOnAddressTagging = 0b0000,
            Fixed = 0b0001,
        ],

        /// Support for the BC instruction
        BC OFFSET(20) NUMBITS(4) [
            NotImplmented = 0b0000,
            Implmented = 0b0001,
        ],

        /// Support for Memory Copy and Memory Set instructions
        MOPS OFFSET(16) NUMBITS(4) [
            NotImplmented = 0b0000,
            Implmented = 0b0001,
        ],

        /// Indicates support for QARMA3 for address authentication
        APA3 OFFSET(12) NUMBITS(4) [
            NotImplemented = 0b0000,
            ImplementedNoEPACNoPAuth2 = 0b0001,
            ImplementedWithEPACNoPAuth2 = 0b0010,
            ImplementedWithEPACAndPauth2 = 0b0011,
            ImplementedNoEPACWithPAuthAndPFAC = 0b0100,
            ImplementedNoEPACWithPAuth2FPACAndFPACCOMBINE = 0b0101,
            ImplementedWithSignedLR = 0b0110,
        ],

        /// Indicates support for QARMA3 for generic code authentication
        GPA3 OFFSET(8) NUMBITS(4) [
            NotImplmented = 0b0000,
            Implmented = 0b0001,
        ],

        /// Support for 12 bits of mantissa in single-precision reciprocal and reciprocal square root instructions
        RPRES OFFSET(4) NUMBITS(4) [
            Mantissa8Bit = 0b0000,
            Mantissa12Bit = 0b0001,
        ],

        /// Support for WFET and WFIT instructions
        WFxT OFFSET(0) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0010,
        ],
    ]
}

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "ID_AA64ISAR2_EL1", "x");
}

pub const ID_AA64ISAR2_EL1: Reg = Reg;
