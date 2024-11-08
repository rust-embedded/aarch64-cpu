use tock_registers::{interfaces::*, register_bitfields};

register_bitfields! {u64,
    pub ICV_CTLR_EL3 [
        Ext  OFFSET(19) NUMBITS(1) [
            WideUnsupported = 0b0, // support int_id 1024 ... 8191
            WideSupported = 0b1,
        ],
        RSS  OFFSET(18) NUMBITS(1) [
            Low = 0b0,
            High = 0b1
        ],
        A3V  OFFSET(15) NUMBITS(1) [
            Disable = 0b0,
            Enable = 0b1,
        ],
        SEIS OFFSET(14) NUMBITS(1) [
            Disable = 0b0,
            Enable = 0b1
        ],
        ID  OFFSET(11) NUMBITS(3) [
            BIT_16 = 0b000,
            BIT_24 = 0b001
        ],
        PRI OFFSET(8) NUMBITS(3) [],
        PMHE OFFSET(6) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0,
        ],
        RM  OFFSET(5) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        EOI_EL1S   OFFSET(4) NUMBITS(1) [
            EOIR_DropAndDeactivate = 0b0,
            EOIR_Drop_Dir_Deactivate = 0b1
        ],
        EOI_EL1NS  OFFSET(3) NUMBITS(1) [
            EOIR_DropAndDeactivate = 0b0,
            EOIR_Drop_Dir_Deactivate = 0b1
        ],
        EOI_EL3 OFFSET(2) NUMBITS(1) [
            EOIR_DropAndDeactivate = 0b0,
            EOIR_Drop_Dir_Deactivate = 0b1
        ],
        CBPR_EL1NS OFFSET(1) NUMBITS(1) [
            Private = 0b0,
            Shared = 0b1,
        ],
        CBPR_EL1S OFFSET(0) NUMBITS(1) [
            Private = 0b0,
            Shared = 0b1,
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ICV_CTLR_EL3::Register;

    sys_coproc_read_raw!(u64, "ICV_CTLR_EL3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ICV_CTLR_EL3::Register;

    sys_coproc_write_raw!(u64, "ICV_CTLR_EL3", "x");
}

pub const ICV_CTLR_EL3: Reg = Reg {};
