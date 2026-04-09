// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - Ali Saidi <alisaidi@amazon.com>
//   - Callum Thomson <callumthom11@gmail.com>

//! AArch64 Memory Model Feature Register 1 - EL1
//!
//! Provides information about the implemented memory model and memory
//! management support in AArch64 state.

use tock_registers::{interfaces::Readable, register_bitfields};

register_bitfields! {u64,
    pub ID_AA64MMFR1_EL1 [
        /// Support for restrictions on branch history speculation around exceptions
        ECBHB OFFSET(60) NUMBITS(4) [
            Undisclosed = 0b0000,
            CannotBeUsed = 0b0001,
        ],

        /// Support for cache maintencance instruction permission
        CMOW OFFSET(56) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for SCTLR_EL1.TIDCP and SCTLR_EL2.TIDCP
        TIDCP1 OFFSET(52) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for intermediate caching of translation table walks
        nTLBPA OFFSET(48) NUMBITS(4) [
            MayIncludeNonCoherentCaches = 0b0000,
            DoesNotIncludeNonCoherentCaches = 0b0001,
        ],

        /// Support for FPCR.{AH, FIZ, NEP}
        AFP OFFSET(44) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Support for HCRX_EL2
        HCX OFFSET(40) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Support for Enhanced Translation Synchronization
        ETS OFFSET(36) NUMBITS(4) [
            NotSupported = 0b0000,
            NotSupported2 = 0b0001,
            ETS2 = 0b0010,
            ETS3 = 0b0011,
        ],

        /// Support for configurable trapping delay of WFE instructions
        TWED OFFSET(32) NUMBITS(4) [
            /// Delaying the trapping of WFE instructions isn't supported
            Unsupported = 0b0000,
            /// Delaying the trapping of WFE instructions is supported
            Supported = 0xb0001,
        ],

        /// Execute-never control at stage2 is distinct for EL0 and EL1
        XNX OFFSET(28) NUMBITS(4) [
            /// There are not distinct stage2 execute never controls for EL1 and EL0
            Unsupported = 0b0000,
            /// There are distinct stage2 execute never controls for EL1 and EL0
            Supported = 0xb0001,
        ],

        /// Speculative reads can generate SError Interrupts
        SpecSEI OFFSET(24) NUMBITS(4) [
            /// PE never generates SError interrupts on a speculative read
            Never = 0b0000,
            /// PE may generate SError interrupts on a speculative read
            Maybe = 0b0001
        ],

        /// Privileged Access Never support
        PAN OFFSET(20) NUMBITS(4) [
            /// Privileged Access Never isn't supported
            Unsupported = 0b0000,
            /// Privileged Access Never is supported
            Supported = 0xb0001,
            /// Privileged Access Never is supported along with AT instruction support
            SupportedAT = 0xb0010,
            /// Enhanced Privileged Access Never is supported
            SupportedEPAN = 0xb0011,
        ],

        /// Limited Ordered regions support
        LO OFFSET(16) NUMBITS(4) [
            /// Limited Ordered regions aren't supported
            Unsupported = 0b0000,
            /// Limited Ordered regions are supported
            Supported = 0xb0001,
        ],

        /// Hierarchical Permission can be disabled in TCRs
        HPDS OFFSET(12) NUMBITS(4) [
            /// HPDS aren't supported
            Unsupported = 0b0000,
            /// HPDS are supported
            Supported = 0xb0001,
        ],

        /// Virtualization Host Extensions
        VH OFFSET(8) NUMBITS(4) [
            /// Virtualization Host Extensions aren't supported
            Unsupported = 0b0000,
            /// Virtualization Host Extensions are supported
            Supported = 0xb0001,
        ],

        /// Number of VMID bits that are supported
        VMIDBits OFFSET(4) NUMBITS(4) [
            /// 8 bits of VMID are supported
            Bits8 = 0b0000,
            /// 16 bits of VMID are supported
            Bits16 = 0b0010,
        ],

        /// Hardware updates to Access and Dirty flags in translation tables
        HAFDBS OFFSET(0) NUMBITS(4) [
            /// Not supported
            Unsupported = 0b0000,
            /// Access flag is supported
            AccessOnly = 0xb0001,
            /// Access and dirty flags are supported
            AccessDirty = 0b0010,
        ],
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ID_AA64MMFR1_EL1::Register;

    sys_coproc_read_raw!(u64, "ID_AA64MMFR1_EL1", "x");
}

pub const ID_AA64MMFR1_EL1: Reg = Reg;
