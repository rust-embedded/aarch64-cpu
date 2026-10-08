// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! SME Feature ID Register 0 - EL1

use tock_registers::interfaces::Readable;
use tock_registers::register_bitfields;

pub struct Reg;

register_bitfields! {u64,
    pub ID_AA64SMFR0_EL1 [
        /// Support for Advanced SIMD and SVE instructions in Streaming SVE
        FA64 OFFSET(63) NUMBITS(1) [
            OnlyDefinedLegal = 0b0,
            AllLegal = 0b1,
        ],

        /// Support for additional SME2 lookup table instructions
        LUTv2 OFFSET(60) NUMBITS(1) [
            NotImplemented = 0b0,
            Implemented = 0b1,
        ],

        /// Support for SME instructions
        SMEver OFFSET(56) NUMBITS(4) [
            MandatorySME = 0b0000,
            UpToSME2 = 0b0001,
            UpToSME21 = 0b0010,
            UpToSME22 = 0b0011,
        ],

        /// Support for SME instructions that accumulate into 64-bit integer elements in the ZA array
        I16I64 OFFSET(52) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b1111,
        ],

        /// Support for SME instructions that accumulate into double-precision floating-point elements in the ZA array
        F64F64 OFFSET(48) NUMBITS(1) [
            NotImplemented = 0b0,
            Implemented = 0b1,
        ],

        /// Support for SME2 `SMOPA`, `SMOPS`, `UMOPA` and `UMOPS`
        I16I32 OFFSET(44) NUMBITS(4) [
            NotImplemented = 0b0,
            Implemented = 0b1,
        ],

        /// Support for SME `BFADD`, `BFMLA`, `BFMLS`, `BFMOPA`, `BFMOPS` and `BFSUB`
        B16B16 OFFSET(43) NUMBITS(1) [
            NotImplemented = 0b0,
            Implemented = 0b1,
        ],

        /// Support for SME2 `FMOPA`, `FMOPS`, `FADD`, `FMLA`, `FMLS`, `FSUB`, `FCVT` and `FCVTL`
        F16F16 OFFSET(42) NUMBITS(1) [
            NotImplemented = 0b0,
            Implemented = 0b1,
        ],

        /// Support for the following SME2 instructions:
        /// ZA-targeting FP8 Instructions that accumulate into half-precision floating-point elements
        /// ZA-targeting non-widening half-precision `FADD` and `FSUB`
        F8F16 OFFSET(41) NUMBITS(1) [
            NotImplemented = 0b0,
            Implemented = 0b1,
        ],

        /// Support for SME instructions that accumulate into single-precision floating-point elements
        F8F32 OFFSET(40) NUMBITS(1) [
            NotImplemented = 0b0,
            Implemented = 0b1,
        ],

        /// Support for SME instructions that accumulate 8-bit outer products into 32-bit tiles
        I8I32 OFFSET(36) NUMBITS(4) [
            NotImplemented = 0b0,
            Implemented = 0b1,
        ],

        /// Support for SME `FMOPA` and `FMOPS`
        F16F32 OFFSET(35) NUMBITS(1) [
            NotImplemented = 0b0,
            Implemented = 0b1,
        ],

        /// Support for SME `BFMOPA` and `BFMOPS
        B16F32 OFFSET(34) NUMBITS(1) [
            NotImplemented = 0b0,
            Implemented = 0b1,
        ],

        /// Support for SME `BMOPA` and `BMOPS`
        BI32I32 OFFSET(33) NUMBITS(1) [
            NotImplemented = 0b0,
            Implemented = 0b1,
        ],

        /// Support for SME `FMOPA` and `FMOPS`
        F32F32 OFFSET(32) NUMBITS(1) [
            NotImplemented = 0b0,
            Implemented = 0b1,
        ],

        /// Support for the SVE2 FP8 to single-precision and half-precision multiply-accumulate instructions in Streaming SVE mode
        SF8FMA OFFSET(30) NUMBITS(1) [
            NoEffect = 0b0,
            Implemented = 0b1,
        ],

        /// Support for the SVE2 FP8 to single-precision 4-way dot product instruction in Streaming SVE mode
        SF8DP4 OFFSET(29) NUMBITS(1) [
            NoEffect = 0b0,
            Implemented = 0b1,
        ],

        /// Support for the SV2 FP8 to half-precision 2-way dot product instructions in Streaming SVE mode
        SF8DP2 OFFSET(28) NUMBITS(1) [
            NoEffect = 0b0,
            Implemented = 0b1,
        ],

        /// Support for SVE bit permute instructions in Streaming SVE mode
        SBitPerm OFFSET(25) NUMBITS(1) [
            NoEffect = 0b0,
            Implemented = 0b1,
        ],

        /// Support for SVE AES and 128-bit polynomial multiply long instructions in Streaming SVE mode
        AES OFFSET(24) NUMBITS(1) [
            NoEffect = 0b0,
            Supported = 0b1,
        ],

        /// Support for FEXPA in Streaming SVE mode
        SFEXPA OFFSET(23) NUMBITS(1) [
            NoEffect = 0b0,
            Supported = 0b1,
        ],

        /// Support for some SME Structured sparsity outer product instructions
        STMOP OFFSET(16) NUMBITS(1) [
            NotImplemented = 0b0,
            Implemented = 0b1,
        ],

        /// Support for some SME Quarter-time outer product instructions
        SMOP4 OFFSET(0) NUMBITS(1) [
            NotImplmented = 0b0,
            Implemented = 0b1,
        ],
    ]
}

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "ID_AA64SMFR0_EL1", "x");
}

pub const ID_AA64SMFR0_EL1: Reg = Reg;
