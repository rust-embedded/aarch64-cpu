use tock_registers::register_bitfields;
register_bitfields!{u64,
    VADescriptor [
        L0 OFFSET(39) NUMBITS(9) [],
        L1 OFFSET(30) NUMBITS(9) [],
        L2 OFFSET(21) NUMBITS(9) [],
        L3 OFFSET(12) NUMBITS(9) [],
        OFFSET OFFSET(0) NUMBITS(12) []
    ]
}

#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct VirtAddress(pub u64);

#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct PhyAddress(pub u64);

pub struct VirtLayout{
    pub indexes: [u64;4],
    pub offset: u64
}

impl VirtAddress {
    pub fn layout(&self) -> VirtLayout {
        VirtLayout {
            indexes: [
                VADescriptor::L0.read(self.0),
                VADescriptor::L1.read(self.0),
                VADescriptor::L2.read(self.0),
                VADescriptor::L3.read(self.0)
            ],
            offset: VADescriptor::OFFSET.read(self.0)
        }
    }
}

impl From<u64> for VirtAddress{
    fn from(value: u64) -> Self {
        VirtAddress(value)
    }
}

impl From<u64> for PhyAddress {
    fn from(value: u64) -> Self {
        PhyAddress(value)
    }
}
