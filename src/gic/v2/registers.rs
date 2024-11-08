use tock_registers::*;

register_bitfields! {u32,
    pub GICD_CTLR [
        ENGrp1 OFFSET(0) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        ENGrp0 OFFSET(0) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ]
    ],
    pub GICD_TYPER [
        LSPI OFFSET(11) NUMBITS(5) [],
        SecurityExtn OFFSET(10) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        CPUNumber OFFSET(5) NUMBITS(3) [],
        ITLines OFFSET(0) NUMBITS(5) []
    ],
    pub GICD_IIDR [
        Variant OFFSET(16) NUMBITS(4) [],
        Revision OFFSET(12) NUMBITS(4) [],
        Implementer OFFSET(0) NUMBITS(12) []
    ],
    pub GICD_ICFGR [
        IntConfig OFFSET(1) NUMBITS(1) [

        ],
        Trigger  OFFSET(0) NUMBITS(1)[
            LevelSensative = 0b0,
            EdgeTriggered = 0b1
        ]
    ],
    pub GICD_NSCAR [
        ACCESS OFFSET(0) NUMBITS(2) [
            None = 0b00,
            RO = 0b01,
            WO = 0b10,
            WR = 0b11
        ]
    ],
    pub GICC_CTLR [
        EOI OFFSET(9) NUMBITS(1) [
            EOIR_DIR = 1,
            EOIR_ONLY = 0
        ],
        FIQBYPDISGRP1 OFFSET(6) NUMBITS(1) [
            Signal = 1,
            NoSignal = 0
        ],
        IRQBYPDISGRP1 OFFSET(5) NUMBITS(1) [
    // Bypass FIQ signal whether or not still affects CPU
    // execution even if the gic is turned off
            Signal = 1,
            NoSignal = 0,
        ],
        EN OFFSET(0) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
    ],
    pub GICC_PMR [
        PRIORITY OFFSET(0) NUMBITS(7)[

        ]
    ],
    pub GICC_BPR [
        BP OFFSET(0) NUMBITS(2) [

        ]
    ],
    pub GICC_GENERIC [
        CPUID OFFSET(10) NUMBITS(2) [

        ],
        INTRID OFFSET(0) NUMBITS(10) [

        ],
    ],
}
