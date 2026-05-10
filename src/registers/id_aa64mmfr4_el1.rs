// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! AArch64 Memory Model Feature Register 4 - EL1

use tock_registers::interfaces::Readable;
use tock_registers::register_bitfields;

pub struct Reg;

register_bitfields! {u64,
    pub ID_AA64MMFR4_EL1 [
        /// Support for SCR_EL3
        SCRX OFFSET(56) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Support for some bitwise write masks
        SRMASK OFFSET(44) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
            SupportedAndSRMASK2 = 0b0010,
        ],

        /// Support for TLBI Domains
        TLBID OFFSET(40) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Delegated SError exceptions from EL3
        E3DSE OFFSET(36) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for Enhanced Abort Syndrome
        EAESR OFFSET(32) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// RME Granular Data Isolation
        RMEGDI OFFSET(28) NUMBITS(4) [
            Reserved = 0b0000,
            ReservedOrNoAccess = 0b0001,
        ],

        /// Support for programming HCR_EL2.E2H
        E2H0 OFFSET(24) NUMBITS(4) [
            Implemented = 0b0000,
            NotImplementedNV1IsRES0 = 0b1110,
            NotImplemented = 0b1111,
        ],

        /// Support for a subset of FEAT_NV and FEAT_NV2
        NV_frac OFFSET(20) NUMBITS(4) [
            AsInAA64MMFR2_EL1 = 0b0000,
            Partial = 0b0001,
            PartialWithStateful = 0b0010,
            PartialWithStatefulAndNV3 = 0b0011,
        ],

        /// Support for Fine Grained Write Trap EL3
        FGWTE3 OFFSET(16) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Support for Hardware accelerator for cleaning Dirty state
        HACDBS OFFSET(12) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Support for concurrent use of two ASIDs
        ASID2 OFFSET(8) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Early Implicit Error Synchronization event
        EIESB OFFSET(4) NUMBITS(4) [
            ImplementedEventBefore = 0b1111,
            NotImplemented = 0b0000,
            ImplementedEventAfter = 0b0001,
            ImplmentedEventAfterAndEL1AndEL2 = 0b0010,
        ],

        /// Support for the clean and invalidate to Point of Physical Storage instructions
        PoPS OFFSET(0) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],
    ]
}

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "ID_AA64MMFR4_EL1", "x");
}

pub const ID_AA64MMFR4_EL1: Reg = Reg;
