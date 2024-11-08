use tock_registers::*;

register_bitfields! {u32,
    pub GICD_CTLR [
        ARE_NS OFFSET(5) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        ARE_S OFFSET(4) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        ENGrp1S  OFFSET(2) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        ENGrp1NS OFFSET(1) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        ENGrp0 OFFSET(0) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ]
    ],
    pub GICD_TYPER [
        LSPI OFFSET(17) NUMBITS(5) [],
        SecurityExtn OFFSET(10) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        CPUNumber OFFSET(5) NUMBITS(3) [],
        ITLines OFFSET(0) NUMBITS(5) []
    ],
    pub GICR_WAKER [
        ChildrenAsleep OFFSET(2) NUMBITS(1) [],
        ProcessorSleep OFFSET(1) NUMBITS(1) [
            NotinLowState = 0b0,
            LowState = 0b1
        ],
        IMPDEF OFFSET(0) NUMBITS(1) [],
    ],
}
register_bitfields! {u64,
    pub GICR_TYPER [ 
        Last OFFSET(4) NUMBITS(1) [
            NotLast = 0b0,
            Last = 0b1,
        ]
    ]
}
