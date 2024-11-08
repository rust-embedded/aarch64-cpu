// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - tsemo4917 <tsemo4917@users.noreply.github.com>
//   - Yiyang Wu <toolmanp@tlmp.cc>

//! Counter-timer Hypervisor Physical Timer Control Register - EL2
//!
//! Control register for the EL2 physical timer.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub CNTHP_CTL_EL2 [
        /// The status of the timer. This bit indicates whether the timer condition is met:
        ///
        /// 0 Timer condition is not met.
        /// 1 Timer condition is met.
        ///
        /// When the value of the ENABLE bit is 1, ISTATUS indicates whether the timer condition is
        /// met. ISTATUS takes no account of the value of the IMASK bit. If the value of ISTATUS is
        /// 1 and the value of IMASK is 0 then the timer interrupt is asserted.
        ///
        /// When the value of the ENABLE bit is 0, the ISTATUS field is UNKNOWN.
        ///
        /// This bit is read-only.
        ISTATUS OFFSET(2) NUMBITS(1) [],

        /// Timer interrupt mask bit. Permitted values are:
        ///
        /// 0 Timer interrupt is not masked by the IMASK bit.
        /// 1 Timer interrupt is masked by the IMASK bit.
        IMASK   OFFSET(1) NUMBITS(1) [
            Masked = 1,
            Unmasked = 0,
        ],

        /// Enables the timer. Permitted values are:
        ///
        /// 0 Timer disabled.
        /// 1 Timer enabled.
        ENABLE  OFFSET(0) NUMBITS(1) [
            Enabled = 1,
            Disabled = 0
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CNTHP_CTL_EL2::Register;

    sys_coproc_read_raw!(u64, "CNTHP_CTL_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CNTHP_CTL_EL2::Register;

    sys_coproc_write_raw!(u64, "CNTHP_CTL_EL2", "x");
}

pub const CNTHP_CTL_EL2: Reg = Reg {};
