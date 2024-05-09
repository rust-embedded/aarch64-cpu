use tock_registers::{
    register_bitfields,
    fields::{FieldValue, Field},
    RegisterLongName,
};
use super::{address::PageMode, MMType};

register_bitfields! {u64,
    pub BlockDescriptor [
        UXN_XN OFFSET(54) NUMBITS(1) [
            TRUE = 0b1,
            FALSE = 0b0
        ],
        PXN OFFSET(53) NUMBITS(1) [
            TRUE = 0b1,
            FALSE = 0b0
        ],
        CONTIGUOUS OFFSET(52) NUMBITS(1) [
            TRUE = 0b1,
            FALSE = 0b0
        ],
        OUTPUT OFFSET(0) NUMBITS(48) [],
        LOWER_ATTRS OFFSET(0) NUMBITS(12) [],
        NSE_NG OFFSET(11) NUMBITS(1) [
            TRUE = 0b1,
            FALSE = 0b0
        ],
        AF OFFSET(10) NUMBITS(1) [
            TRUE = 0b1,
            FALSE = 0b0
        ],
        SH OFFSET(8) NUMBITS(2) [
            NS = 0b00,
            RESERVED = 0b01,
            OS = 0b10,
            IS = 0b11
        ],
        AP OFFSET(6) NUMBITS(2) [
            RW_ELx_None_EL0 = 0b00,
            RW_ELx_RW_EL0 = 0b01,
            RO_ELx_None_EL0 = 0b10,
            RO_ELx_RO_EL0 = 0b11
        ],
        S2AP OFFSET(6) NUMBITS(2) [
            NONE = 0b00,
            RO = 0b01,
            WO = 0b10,
            WR = 0b11,
        ],
        NS OFFSET(5) NUMBITS(1) [
            TRUE = 0b1,
            FALSE = 0b0
        ],
        ATTR OFFSET(2) NUMBITS(2) [

        ],
        TYPE OFFSET(1) NUMBITS(1) [
            BLOCK = 0b0,
            PAGE = 0b1
        ],
        VALID OFFSET(0) NUMBITS(1) [
            TRUE = 0b1,
            FALSE = 0b0
        ]
    ]
}

register_bitfields! {u64,
    pub TableDescriptor [
        NS OFFSET(63) NUMBITS(1) [
            TRUE = 0b1,
            FALSE = 0b0
        ],
        AP OFFSET(61) NUMBITS(2) [
            NO_EFFECT = 0b00,
            UNPRIV_RESTRICTED = 0b01,
            WR_RESTRICTED = 0b10,
            WR_UNPRIV_RD_RESTRICTED = 0b11
        ],
        UXN_XN OFFSET(60) NUMBITS(1) [
            TRUE = 0b1,
            FALSE = 0b0
        ],
        PXN OFFSET(59) NUMBITS(1) [
            TRUE = 0b1,
            FALSE = 0b0
        ],
        OUTPUT OFFSET(0) NUMBITS(48) [],
        LOWER_ATTRS OFFSET(0) NUMBITS(2) [],
        TYPE OFFSET(1) NUMBITS(1) [
            TABLE = 0b1,
            BLOCK = 0b0
        ],
        VALID OFFSET(0) NUMBITS(1) [
            TRUE = 0b1,
            FALSE = 0b0
        ]
    ]
}

pub trait Descriptor {}

impl Descriptor for BlockDescriptor::Register {}
impl Descriptor for TableDescriptor::Register {}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct VADescriptor(pub u64);

impl VADescriptor {
    pub fn read(&self) -> u64 {
        self.0
    }

    pub fn read_field<T>(&self, field: Field<u64, T>) -> u64
    where
        T: RegisterLongName,
    {
        field.read(self.0)
    }

    pub fn write(&mut self, val: u64) {
        self.0 = val;
    }

    pub fn write_field<T>(&mut self, field_value: FieldValue<u64, T>)
    where
        T: RegisterLongName,
    {
        self.0 = field_value.modify(self.0);
    }

    pub fn output(&self) -> u64 {
        TableDescriptor::OUTPUT.read(self.0) & !TableDescriptor::LOWER_ATTRS.mask
    }

    pub fn table(output: u64) -> u64 {
        (TableDescriptor::OUTPUT.val(output)
            + TableDescriptor::TYPE::TABLE
            + TableDescriptor::VALID::TRUE)
            .value
    }

    pub fn block(output: u64, typ: MMType, mode: PageMode) -> u64 {
        (BlockDescriptor::OUTPUT.val(output) + typ.into() + mode.into()).value
    }
}

impl From<u64> for VADescriptor {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<VADescriptor> for u64 {
    fn from(value: VADescriptor) -> Self {
        value.0
    }
}
