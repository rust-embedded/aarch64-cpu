// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2025 by the author(s)
//
// Author(s):
//   - Yan Tan <tanyan@kylinos.cn>

//! Extended Translation Control Register - EL1
//!
//! Provides control over the translation table walks for stage 1 of the EL1&0 translation regime.
//! This register is present only when FEAT_TCR2 is implemented and FEAT_AA64 is implemented.
//! Otherwise, direct accesses to TCR2_EL1 are UNDEFINED.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub TCR2_EL1 [
        /// Reserved, RES0，[63:22]

        /// FNGNA1 - Force non-global for unassured translations using TTBR1_EL1
        /// When FEAT_THE is implemented:
        /// - Controls whether translations using TTBR1_EL1 are treated as non-global
        /// - Only effective when FEAT_THE is implemented and specific conditions are met
        FNGNA1 OFFSET(21) NUMBITS(1) [],

        /// FNGNA0 - Force non-global for unassured translations using TTBR0_EL1
        /// When FEAT_THE is implemented:
        /// - Controls whether translations using TTBR0_EL1 are treated as non-global
        /// - Only effective when FEAT_THE is implemented and specific conditions are met
        FNGNA0 OFFSET(20) NUMBITS(1) [],

        /// Reserved, RES0，[19]

        /// FNG1 - Force non-global translations for TTBR1_EL1
        /// When FEAT_ASID2 is implemented:
        /// - Forces all translations using TTBR1_EL1 to be non-global
        /// - Only effective when FEAT_ASID2 is implemented
        FNG1 OFFSET(18) NUMBITS(1) [],

        /// FNG0 - Force non-global translations for TTBR0_EL1
        /// When FEAT_ASID2 is implemented:
        /// - Forces all translations using TTBR0_EL1 to be non-global
        /// - Only effective when FEAT_ASID2 is implemented
        FNG0 OFFSET(17) NUMBITS(1) [],

        /// A2 - Enable use of two ASIDs
        /// When FEAT_ASID2 is implemented:
        /// - Controls whether two ASIDs can be used
        /// - Only effective when FEAT_ASID2 is implemented
        ///  Use of two ASIDs is disabled
        ///  Use of two ASIDs is enabled
        A2 OFFSET(16) NUMBITS(1) [],

        /// Disable the Contiguous bit for the Start Table for TTBR1_EL1
        /// When FEAT_D128 is implemented and TCR2_EL1.D128 == 1:
        /// - Controls whether the Contiguous bit is treated as 0 for TTBR1_EL1
        /// - Only effective when FEAT_D128 is implemented and D128 is enabled
        DisCH1 OFFSET(15) NUMBITS(1) [],

        /// Disable the Contiguous bit for the Start Table for TTBR0_EL1
        /// When FEAT_D128 is implemented and TCR2_EL1.D128 == 1:
        /// - Controls whether the Contiguous bit is treated as 0 for TTBR0_EL1
        /// - Only effective when FEAT_D128 is implemented and D128 is enabled
        DisCH0 OFFSET(14) NUMBITS(1) [],

        /// Reserved, RES0，[13:12]

        /// Hardware managed Access Flag for Table descriptors
        /// When FEAT_HAFT is implemented:
        /// - Controls whether hardware manages the Access Flag for Table descriptors
        /// - Only effective when FEAT_HAFT is implemented
        HAFT OFFSET(11) NUMBITS(1) [
            /// Hardware managed Access Flag for Table descriptors is disabled
            Disabled = 0,
            /// Hardware managed Access Flag for Table descriptors is enabled
            Enabled = 1
        ],

        /// Permit Translation table walk Incoherence
        /// When FEAT_THE is implemented:
        /// - Controls whether RCWS instructions can generate writes with Reduced Coherence
        /// - Only effective when FEAT_THE is implemented
        PTTWI OFFSET(10) NUMBITS(1) [],

        /// Reserved, RES0，[9:6]

        /// Enables VMSAv9-128 translation system
        /// When FEAT_D128 is implemented:
        /// - Controls whether the translation system follows VMSAv9-128 process
        /// - Only effective when FEAT_D128 is implemented
        /// 0b0: Translation system follows VMSAv8-64 translation process
        /// 0b1: Translation system follows VMSAv9-128 translation process
        D128 OFFSET(5) NUMBITS(1) [],

        /// Enable Attribute Indexing Extension
        /// When FEAT_AIE is implemented:
        /// - Controls whether Attribute Indexing Extension is enabled
        /// - Only effective when FEAT_AIE is implemented
        /// - RES1 when TCR2_EL1.D128 is 1
        AIE OFFSET(4) NUMBITS(1) [
            /// Attribute Indexing Extension Disabled
            Disabled = 0,
            /// Attribute Indexing Extension Enabled
            Enabled = 1
        ],

        /// Enables Permission Overlays for privileged accesses
        /// When FEAT_S1POE is implemented:
        /// - Controls whether Permission Overlays are enabled for EL1 access
        /// - Only effective when FEAT_S1POE is implemented
        /// - Not permitted to be cached in a TLB
        POE OFFSET(3) NUMBITS(1) [
            /// Permission overlay disabled for EL1 access
            Disabled = 0,
            /// Permission overlay enabled for EL1 access
            Enabled = 1
        ],

        /// Enables Permission Overlays for unprivileged accesses
        /// When FEAT_S1POE is implemented:
        /// - Controls whether Permission Overlays are enabled for EL0 access
        /// - Only effective when FEAT_S1POE is implemented
        /// - Not permitted to be cached in a TLB
        E0POE OFFSET(2) NUMBITS(1) [
            /// Permission overlay disabled for EL0 access
            Disabled = 0,
            /// Permission overlay enabled for EL0 access
            Enabled = 1
        ],

        /// Enables usage of Indirect Permission Scheme
        /// When FEAT_S1PIE is implemented:
        /// - Controls whether Indirect Permission Scheme is used
        /// - Only effective when FEAT_S1PIE is implemented
        /// - RES1 when TCR2_EL1.D128 is 1
        PIE OFFSET(1) NUMBITS(1) [
            /// Direct permission model
            Direct = 0,
            /// Indirect permission model
            Indirect = 1
        ],

        /// Protected attribute enable
        /// When FEAT_THE is implemented:
        /// - Controls whether bit[52] is used as the Protected bit
        /// - Only effective when FEAT_THE is implemented
        /// - RES0 when TCR2_EL1.D128 is 1
        PnCH OFFSET(0) NUMBITS(1) [
            /// bit[52] is not the Protected bit
            NotProtected = 0,
            /// bit[52] is the Protected bit
            Protected = 1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = TCR2_EL1::Register;

    sys_coproc_read_raw!(u64, "S3_0_C2_C0_3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = TCR2_EL1::Register;

    sys_coproc_write_raw!(u64, "S3_0_C2_C0_3", "x");
}

pub const TCR2_EL1: Reg = Reg {};
