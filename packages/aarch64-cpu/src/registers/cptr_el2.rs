// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2024 by the author(s)
//
// Author(s):
//   - Sangwan Kwon <sangwan.kwon@samsung.com>

//! Architectural Feature Trap Register - EL2
//!
//! Controls trapping to EL2 of accesses to CPACR, CPACR_EL1, trace, Activity Monitor, SME,
//! Streaming SVE, SVE, and Advanced SIMD and floating-point functionality.
//!
//! The fields below describe the layout when `HCR_EL2.E2H` is 0. When `HCR_EL2.E2H` is 1
//! the register uses the `CPACR_EL1`-like `FPEN`/`ZEN`/`SMEN` layout instead.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub CPTR_EL2 [
        /// Traps EL1 accesses to `CPACR_EL1` (and AArch32 `CPACR`) to EL2, when EL2 is
        /// enabled in the current Security state. The exception is reported using
        /// `ESR_EL2.EC` value 0x18 (0x03 from AArch32).
        TCPAC OFFSET(31) NUMBITS(1) [
            /// This control does not cause any instructions to be trapped.
            NoTrap = 0b0,
            /// EL1 accesses to `CPACR_EL1` are trapped to EL2.
            Trap = 0b1
        ],

        /// Trap Activity Monitor access. Traps EL1 and EL0 accesses to all Activity Monitor
        /// registers to EL2.
        ///
        /// 0 Accesses from EL1 and EL0 to Activity Monitor registers are not trapped.
        ///
        /// 1 Accesses from EL1 and EL0 to Activity Monitor registers are trapped to EL2,
        /// when EL2 is enabled in the current Security state.
        ///
        /// RES0 when FEAT_AMUv1 is not implemented.
        TAM  OFFSET(30) NUMBITS(1) [],

        /// Traps System register accesses to all implemented trace registers from both
        /// Execution states to EL2, when EL2 is enabled in the current Security state.
        /// The exception is reported using `ESR_EL2.EC` value 0x18 (0x05 from AArch32).
        ///
        /// TTA is bit number 28 when `HCR_EL2.E2H` is 1.
        TTA OFFSET(20) NUMBITS(1) [
            /// This control does not cause any instructions to be trapped.
            NoTrap = 0b0,
            /// System register accesses to trace registers are trapped to EL2.
            Trap = 0b1
        ],

        /// Reserved, RES1.
        RES1_13 OFFSET(13) NUMBITS(1) [],

        /// Traps execution of SME and Streaming SVE instructions, and accesses to `SMCR_EL2`,
        /// `SMCR_EL1` and `SVCR`, at EL2, EL1 and EL0 to EL2, when EL2 is enabled in the
        /// current Security state. The exception is reported using `ESR_EL2.EC` value 0x1D.
        ///
        /// RES1 when FEAT_SME is not implemented.
        TSM OFFSET(12) NUMBITS(1) [
            /// This control does not cause any instructions to be trapped.
            NoTrap = 0b0,
            /// SME instructions and register accesses are trapped to EL2.
            Trap = 0b1
        ],

        /// Traps execution of instructions which access the Advanced SIMD and floating-point
        /// functionality, from both Execution states, to EL2, when EL2 is enabled in the
        /// current Security state. The exception is reported using `ESR_EL2.EC` value 0x07.
        ///
        /// A trap taken as a result of [`CPTR_EL2::TZ`] or [`CPTR_EL2::TSM`] has precedence
        /// over a trap taken as a result of `CPTR_EL2::TFP`.
        ///
        /// On a Warm reset, this field resets to an architecturally UNKNOWN value.
        TFP OFFSET(10) NUMBITS(1) [
            /// This control does not cause any instructions to be trapped.
            NoTrap = 0b0,
            /// Advanced SIMD and floating-point instructions at EL2, EL1 and EL0 are
            /// trapped to EL2.
            Trap = 0b1
        ],

        /// Reserved, RES1.
        RES1_9 OFFSET(9) NUMBITS(1) [],

        /// Traps execution at EL2, EL1 and EL0 of SVE instructions, and EL2 and EL1 accesses
        /// to `ZCR_EL2` and `ZCR_EL1`, to EL2, when EL2 is enabled in the current Security
        /// state. The exception is reported using `ESR_EL2.EC` value 0x19.
        ///
        /// RES1 when FEAT_SVE is not implemented.
        TZ OFFSET(8) NUMBITS(1) [
            /// This control does not cause any instructions to be trapped.
            NoTrap = 0b0,
            /// SVE instructions and register accesses are trapped to EL2.
            Trap = 0b1
        ],

        /// Reserved, RES1.
        RES1_7_0 OFFSET(0) NUMBITS(8) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CPTR_EL2::Register;

    sys_coproc_read_raw!(u64, "CPTR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CPTR_EL2::Register;

    sys_coproc_write_raw!(u64, "CPTR_EL2", "x");
}

pub const CPTR_EL2: Reg = Reg {};
