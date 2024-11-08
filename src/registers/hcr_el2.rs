// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>
//   - Bradley Landherr <landhb@users.noreply.github.com>
//   - Javier Alvarez <javier.alvarez@allthingsembedded.com>

//! Hypervisor Configuration Register - EL2
//!
//! Provides configuration controls for virtualization, including defining
//! whether various Non-secure operations are trapped to EL2.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub HCR_EL2 [
        /// When FEAT_S2FWB is implemented Forced Write-back changes the combined cachability of stage1
        /// and stage2 attributes
        FWB OFFSET(46) NUMBITS(1) [
           /// Stage2 memory type and cacheability attributes are in bits[5:2] of the stage2 PTE
           Disabled = 0,
           /// Stage1 memory type can be overridden by Stage2 descriptor
           Enabled = 1,
        ],

        /// Controls the use of instructions related to Pointer Authentication:
        ///
        ///   - In EL0, when HCR_EL2.TGE==0 or HCR_EL2.E2H==0, and the associated SCTLR_EL1.En<N><M>==1.
        ///   - In EL1, the associated SCTLR_EL1.En<N><M>==1.
        ///
        /// Traps are reported using EC syndrome value 0x09. The Pointer Authentication instructions
        /// trapped are:
        ///
        /// `AUTDA`, `AUTDB`, `AUTDZA`, `AUTDZB`, `AUTIA`, `AUTIA1716`, `AUTIASP`, `AUTIAZ`, `AUTIB`, `AUTIB1716`,
        /// `AUTIBSP`, `AUTIBZ`, `AUTIZA`, `AUTIZB`, `PACGA`, `PACDA`, `PACDB`, `PACDZA`, `PACDZB`, `PACIA`,
        /// `PACIA1716`, `PACIASP`, `PACIAZ`, `PACIB`, `PACIB1716`, `PACIBSP`, `PACIBZ`, `PACIZA`, `PACIZB`,
        /// `RETAA`, `RETAB`, `BRAA`, `BRAB`, `BLRAA`, `BLRAB`, `BRAAZ`, `BRABZ`, `BLRAAZ`, `BLRABZ`,
        /// `ERETAA`, `ERETAB`, `LDRAA`, and `LDRAB`.
        API   OFFSET(41) NUMBITS(1) [
            EnableTrapPointerAuthInstToEl2 = 0,
            DisableTrapPointerAuthInstToEl2 = 1
        ],

        /// Trap registers holding "key" values for Pointer Authentication. Traps accesses to the
        /// following registers from EL1 to EL2, when EL2 is enabled in the current Security state,
        /// reported using EC syndrome value 0x18:
        ///
        /// `APIAKeyLo_EL1`, `APIAKeyHi_EL1`, `APIBKeyLo_EL1`, `APIBKeyHi_EL1`, `APDAKeyLo_EL1`,
        /// `APDAKeyHi_EL1`, `APDBKeyLo_EL1`, `APDBKeyHi_EL1`, `APGAKeyLo_EL1`, and `APGAKeyHi_EL1`.
        APK   OFFSET(40) NUMBITS(1) [
            EnableTrapPointerAuthKeyRegsToEl2 = 0,
            DisableTrapPointerAuthKeyRegsToEl2 = 1,
        ],

        /// Route synchronous External abort exceptions to EL2.
        ///   if 0: This control does not cause exceptions to be routed from EL0 and EL1 to EL2.
        ///   if 1: Route synchronous External abort exceptions from EL0 and EL1 to EL2, when EL2 is
        ///         enabled in the current Security state, if not routed to EL3.
        TEA   OFFSET(37) NUMBITS(1) [
            DisableTrapSyncExtAbortsToEl2 = 0,
            EnableTrapSyncExtAbortsToEl2 = 1,
        ],

        /// Trap accesses of Error Record registers. Enables a trap to EL2 on accesses of
        /// Error Record registers.
        ///
        /// 0 Accesses of the specified Error Record registers are not trapped by this mechanism.
        /// 1 Accesses of the specified Error Record registers at EL1 are trapped to EL2,
        ///   unless the instruction generates a higher priority exception.
        TERR  OFFSET(36) NUMBITS(1) [],

        /// Trap LOR registers. Traps Non-secure EL1 accesses to LORSA_EL1, LOREA_EL1, LORN_EL1,
        /// LORC_EL1, and LORID_EL1 registers to EL2.
        ///
        /// 0 This control does not cause any instructions to be trapped.
        /// 1 Non-secure EL1 accesses to the LOR registers are trapped to EL2.
        TLOR  OFFSET(35) NUMBITS(1) [],

        /// EL2 Host. Enables a configuration where a Host Operating System is running in EL2, and
        /// the Host Operating System's applications are running in EL0.
        E2H   OFFSET(34) NUMBITS(1) [
            DisableOsAtEl2 = 0,
            EnableOsAtEl2 = 1
        ],

        /// Execution state control for lower Exception levels:
        ///
        /// 0 Lower levels are all AArch32.
        /// 1 The Execution state for EL1 is AArch64. The Execution state for EL0 is determined by
        ///   the current value of PSTATE.nRW when executing at EL0.
        ///
        /// If all lower Exception levels cannot use AArch32 then this bit is RAO/WI.
        ///
        /// In an implementation that includes EL3, when SCR_EL3.NS==0, the PE behaves as if this
        /// bit has the same value as the SCR_EL3.RW bit for all purposes other than a direct read
        /// or write access of HCR_EL2.
        ///
        /// The RW bit is permitted to be cached in a TLB.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this
        /// field behaves as 1 for all purposes other than a direct read of the value of this bit.
        RW   OFFSET(31) NUMBITS(1) [
            AllLowerELsAreAarch32 = 0,
            EL1IsAarch64 = 1
        ],

        /// Trap General Exceptions, from EL0.
        ///
        /// If enabled:
        ///   - When EL2 is not enabled in the current Security state, this control has no effect on
        ///     execution at EL0.
        ///
        ///   - When EL2 is enabled in the current Security state, in all cases:
        ///
        ///       - All exceptions that would be routed to EL1 are routed to EL2.
        ///       - If EL1 is using AArch64, the SCTLR_EL1.M field is treated as being 0 for all
        ///         purposes other than returning the result of a direct read of SCTLR_EL1.
        ///       - If EL1 is using AArch32, the SCTLR.M field is treated as being 0 for all
        ///         purposes other than returning the result of a direct read of SCTLR.
        ///       - All virtual interrupts are disabled.
        ///       - Any IMPLEMENTATION DEFINED mechanisms for signaling virtual interrupts are
        ///         disabled.
        ///       - An exception return to EL1 is treated as an illegal exception return.
        ///       - The MDCR_EL2.{TDRA, TDOSA, TDA, TDE} fields are treated as being 1 for all
        ///         purposes other than returning the result of a direct read of MDCR_EL2.
        ///
        ///   - In addition, when EL2 is enabled in the current Security state, if:
        ///
        ///       - HCR_EL2.E2H is 0, the Effective values of the HCR_EL2.{FMO, IMO, AMO} fields
        ///         are 1.
        ///       - HCR_EL2.E2H is 1, the Effective values of the HCR_EL2.{FMO, IMO, AMO} fields
        ///         are 0.
        ///
        ///   - For further information on the behavior of this bit when E2H is 1, see 'Behavior of
        ///     HCR_EL2.E2H'.
        TGE   OFFSET(27) NUMBITS(1) [
            DisableTrapGeneralExceptionsToEl2 = 0,
            EnableTrapGeneralExceptionsToEl2 = 1,
        ],

        /// Trap Virtual Memory controls, from EL0.
        TVM   OFFSET(26) NUMBITS(1) [
            DisableTrapVMCtrlRegistersToEl2 = 0,
            EnableTrapVMCtrlRegistersToEl2 = 1,
        ],

        /// Trap data or unified cache maintenance instructions that operate by Set/Way.
        /// Traps execution of those cache maintenance instructions at EL1 to EL2, when
        /// EL2 is enabled in the current Security state.
        ///
        /// 0 This control does not cause any instructions to be trapped.
        /// 1 Execution of the specified instructions is trapped to EL2, when EL2 is enabled
        /// in the current Security state.
        TSW   OFFSET(22) NUMBITS(1) [],

        /// Trap Auxiliary Control Registers. Traps EL1 accesses to the Auxiliary Control Registers
        /// to EL2, when EL2 is enabled in the current Security state
        ///
        /// 0 This control does not cause any instructions to be trapped.
        /// 1 EL1 accesses to the specified registers are trapped to EL2, when EL2 is enabled in the
        ///   current Security state.
        TACR  OFFSET(21) NUMBITS(1) [],

        /// Trap IMPLEMENTATION DEFINED functionality. Traps EL1 accesses to the encodings reserved
        /// for IMPLEMENTATION DEFINED functionality to EL2, when EL2 is enabled in the current
        /// Security state
        ///
        /// 0 This control does not cause any instructions to be trapped.
        /// 1 EL1 accesses to or execution of the specified encodings reserved for IMPLEMENTATION
        /// DEFINED functionality are trapped to EL2, when EL2 is enabled in the current Security
        /// state.
        TIDCP OFFSET(20) NUMBITS(1) [],

        /// Trap ID group 3. Traps EL1 reads of group 3 ID registers to EL2, when EL2 is enabled
        /// in the current Security state.
        ///
        /// 0 This control does not cause any instructions to be trapped.
        /// 1 The specified EL1 read accesses to ID group 3 registers are trapped to EL2, when EL2
        /// is enabled in the current Security state.
        TID3  OFFSET(18) NUMBITS(1) [],

        /// Trap SMC instructions. Traps EL1 execution of SMC instructions to EL2, when EL2 is
        /// enabled in the current Security state.
        ///
        /// If execution is in AArch64 state, the trap is reported using EC syndrome value 0x17.
        ///
        /// HCR_EL2.TSC traps execution of the SMC instruction. It is not a routing control for
        /// the SMC exception. Trap exceptions and SMC exceptions have different preferred return
        /// addresses.
        ///
        /// If disabled, this control does not cause any instructions to be trapped.
        ///
        /// If enabled:
        ///   - If EL3 is implemented, then any attempt to execute an SMC instruction at EL1 is
        ///     trapped to EL2, when EL2 is enabled in the current Security state, regardless of
        ///     the value of SCR_EL3.SMD.
        ///   - If EL3 is not implemented, FEAT_NV is implemented, and HCR_EL2.NV is 1, then any
        ///     attempt to execute an SMC instruction at EL1 using AArch64 is trapped to EL2,
        ///     when EL2 is enabled in the current Security state.
        ///   - If EL3 is not implemented, and either FEAT_NV is not implemented or HCR_EL2.NV is 0,
        ///     then it is IMPLEMENTATION DEFINED whether:
        ///       - Any attempt to execute an SMC instruction at EL1 is trapped to EL2, when EL2 is
        ///         enabled in the current Security state.
        ///       - Any attempt to execute an SMC instruction is UNDEFINED.
        ///
        /// SMC instructions are UNDEFINED at EL0.
        ///
        /// If EL3 is not implemented, and either FEAT_NV is not implemented or HCR_EL2.NV is 0,
        /// then it is IMPLEMENTATION DEFINED whether this bit is:
        ///   - RES0.
        ///   - Implemented with the functionality as described in HCR_EL2.TSC.
        ///
        /// When HCR_EL2.TGE is 1, the PE ignores the value of this field for all purposes other
        /// than a direct read of this field.
        TSC   OFFSET(19) NUMBITS(1) [
            DisableTrapEl1SmcToEl2 = 0,
            EnableTrapEl1SmcToEl2 = 1,
        ],

        TWE OFFSET(15) NUMBITS(1) [
            DisableTrapEl1WfetoEl2 = 0,
            EnableTrapEl1WfetoEl2 = 1
        ],

        /// Default Cacheability.
        ///
        /// 0 This control has no effect on the Non-secure EL1&0 translation regime.
        ///
        /// 1 In Non-secure state:
        ///   - When EL1 is using AArch64, the PE behaves as if the value of the SCTLR_EL1.M field
        ///     is 0 for all purposes other than returning the value of a direct read of SCTLR_EL1.
        ///
        ///   - When EL1 is using AArch32, the PE behaves as if the value of the SCTLR.M field is 0
        ///     for all purposes other than returning the value of a direct read of SCTLR.
        ///
        ///   - The PE behaves as if the value of the HCR_EL2.VM field is 1 for all purposes other
        ///     than returning the value of a direct read of HCR_EL2.
        ///
        ///   - The memory type produced by stage 1 of the EL1&0 translation regime is Normal
        ///     Non-Shareable, Inner Write-Back Read-Allocate Write-Allocate, Outer Write-Back
        ///     Read-Allocate Write-Allocate.
        ///
        /// This field has no effect on the EL2, EL2&0, and EL3 translation regimes.
        ///
        /// This field is permitted to be cached in a TLB.
        ///
        /// In an implementation that includes EL3, when the value of SCR_EL3.NS is 0 the PE behaves
        /// as if this field is 0 for all purposes other than a direct read or write access of
        /// HCR_EL2.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this
        /// field behaves as 0 for all purposes other than a direct read of the value of this field.
        DC   OFFSET(12) NUMBITS(1) [],

        /// Barrier Shareability upgrade. This field determines the minimum shareability domain that
        /// is applied to any barrier instruction executed from EL1 or EL0.
        BSU  OFFSET(10) NUMBITS(2) [
            NoEffect = 0b00,
            InnerShareable = 0b01,
            OuterShareable = 0b10,
            FullSystem = 0b11
        ],

        /// Force broadcast. Causes the following instructions to be broadcast within the Inner
        /// Shareable domain when executed from EL1.
        ///
        /// 0 This field has no effect on the operation of the specified instructions.
        /// 1 When one of the specified instruction is executed at EL1, the instruction is broadcast
        /// within the Inner Shareable shareability domain.
        FB    OFFSET(9) NUMBITS(1) [],

        /// Physical SError interrupt routing.
        ///   - If bit is 1 when executing at any Exception level, and EL2 is enabled in the current
        ///     Security state:
        ///     - Physical SError interrupts are taken to EL2, unless they are routed to EL3.
        ///     - When the value of HCR_EL2.TGE is 0, then virtual SError interrupts are enabled.
        AMO   OFFSET(5) NUMBITS(1) [
            EnableSError = 1,
            DisableSError= 0,
        ],

        /// Physical IRQ Routing.
        ///
        /// If this bit is 0:
        ///   - When executing at Exception levels below EL2, and EL2 is enabled in the current
        ///     Security state:
        ///     - When the value of HCR_EL2.TGE is 0, Physical IRQ interrupts are not taken to EL2.
        ///     - When the value of HCR_EL2.TGE is 1, Physical IRQ interrupts are taken to EL2
        ///       unless they are routed to EL3.
        ///     - Virtual IRQ interrupts are disabled.
        ///
        /// If this bit is 1:
        ///   - When executing at any Exception level, and EL2 is enabled in the current Security
        ///     state:
        ///     - Physical IRQ interrupts are taken to EL2, unless they are routed to EL3.
        ///     - When the value of HCR_EL2.TGE is 0, then Virtual IRQ interrupts are enabled.
        ///
        /// If EL2 is enabled in the current Security state, and the value of HCR_EL2.TGE is 1:
        ///   - Regardless of the value of the IMO bit, physical IRQ Interrupts target EL2 unless
        ///     they are routed to EL3.
        ///   - When FEAT_VHE is not implemented, or if HCR_EL2.E2H is 0, this field behaves as 1
        ///     for all purposes other than a direct read of the value of this bit.
        ///   - When FEAT_VHE is implemented and HCR_EL2.E2H is 1, this field behaves as 0 for all
        ///     purposes other than a direct read of the value of this bit.
        ///
        /// For more information, see 'Asynchronous exception routing'.
        IMO   OFFSET(4) NUMBITS(1) [
            DisableVirtualIRQ = 0,
            EnableVirtualIRQ = 1,
        ],

        /// Physical FIQ Routing.
        /// If this bit is 0:
        ///   - When executing at Exception levels below EL2, and EL2 is enabled in the current
        ///     Security state:
        ///     - When the value of HCR_EL2.TGE is 0, Physical FIQ interrupts are not taken to EL2.
        ///     - When the value of HCR_EL2.TGE is 1, Physical FIQ interrupts are taken to EL2
        ///       unless they are routed to EL3.
        ///     - Virtual FIQ interrupts are disabled.
        ///
        /// If this bit is 1:
        ///   - When executing at any Exception level, and EL2 is enabled in the current Security
        ///     state:
        ///     - Physical FIQ interrupts are taken to EL2, unless they are routed to EL3.
        ///     - When HCR_EL2.TGE is 0, then Virtual FIQ interrupts are enabled.
        ///
        /// If EL2 is enabled in the current Security state and the value of HCR_EL2.TGE is 1:
        ///   - Regardless of the value of the FMO bit, physical FIQ Interrupts target EL2 unless
        ///     they are routed to EL3.
        ///   - When FEAT_VHE is not implemented, or if HCR_EL2.E2H is 0, this field behaves as 1
        ///     for all purposes other than a direct read of the value of this bit.
        ///   - When FEAT_VHE is implemented and HCR_EL2.E2H is 1, this field behaves as 0 for all
        ///     purposes other than a direct read of the value of this bit.
        ///
        /// For more information, see 'Asynchronous exception routing'.
        FMO   OFFSET(3) NUMBITS(1) [
            DisableVirtualFIQ = 0,
            EnableVirtualFIQ = 1,
        ],

        PTW OFFSET(2) NUMBITS(1) [
            DisablePTW = 0,
            EnablePTW = 1
        ],

        /// Set/Way Invalidation Override. Causes Non-secure EL1 execution of the data cache
        /// invalidate by set/way instructions to perform a data cache clean and invalidate by
        /// set/way:
        ///
        /// 0 This control has no effect on the operation of data cache invalidate by set/way
        ///   instructions.
        ///
        /// 1 Data cache invalidate by set/way instructions perform a data cache clean and
        ///   invalidate by set/way.
        ///
        /// When the value of this bit is 1:
        ///
        /// AArch32: DCISW performs the same invalidation as a DCCISW instruction.
        ///
        /// AArch64: DC ISW performs the same invalidation as a DC CISW instruction.
        ///
        /// This bit can be implemented as RES 1.
        ///
        /// In an implementation that includes EL3, when the value of SCR_EL3.NS is 0 the PE behaves
        /// as if this field is 0 for all purposes other than a direct read or write access of
        /// HCR_EL2.
        ///
        /// When HCR_EL2.TGE is 1, the PE ignores the value of this field for all purposes other
        /// than a direct read of this field.
        SWIO OFFSET(1) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Virtualization enable. Enables stage 2 address translation for the EL1&0 translation regime,
        /// when EL2 is enabled in the current Security state. The possible values are:
        ///
        /// 0    EL1&0 stage 2 address translation disabled.
        /// 1    EL1&0 stage 2 address translation enabled.
        ///
        /// When the value of this bit is 1, data cache invalidate instructions executed at EL1 perform
        /// a data cache clean and invalidate. For the invalidate by set/way instruction this behavior
        /// applies regardless of the value of the HCR_EL2.SWIO bit.
        ///
        /// This bit is permitted to be cached in a TLB.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this
        /// field behaves as 0 for all purposes other than a direct read of the value of this field.
        VM OFFSET(0) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = HCR_EL2::Register;

    sys_coproc_read_raw!(u64, "HCR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = HCR_EL2::Register;

    sys_coproc_write_raw!(u64, "HCR_EL2", "x");
}

pub const HCR_EL2: Reg = Reg {};
