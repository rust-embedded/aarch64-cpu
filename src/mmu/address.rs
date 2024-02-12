use tock_registers::register_bitfields;
register_bitfields! {u64,
    VADescriptor [
        L0 OFFSET(39) NUMBITS(9) [],
        L1 OFFSET(30) NUMBITS(9) [],
        L2 OFFSET(21) NUMBITS(9) [],
        L3 OFFSET(12) NUMBITS(9) [],
        OFFSET OFFSET(0) NUMBITS(12) []
    ]
}

pub struct VirtLayout {
    pub indexes: [u64; 4],
    pub offset: u64,
}

pub type MMSegment = (u64, u64);

#[derive(Clone, Copy)]
pub struct MMRegion {
    pub mem: MMSegment,
    pub granule: u64,
}

impl MMRegion {
    pub fn new(segment: MMSegment, granule: u64) -> Self {
        MMRegion {
            mem: (
                segment.0 & (!(granule - 1)),
                (segment.1 + granule - 1) & (!(granule - 1)),
            ),
            granule,
        }
    }

    pub fn inbound(&self, addr: u64) -> bool {
        return self.mem.0 <= addr && addr < self.mem.1;
    }
}

impl Iterator for MMRegion {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.mem.0;
        if self.inbound(ret) {
            self.mem.0 += self.granule;
            Some(ret)
        } else {
            None
        }
    }
}

impl From<u64> for VirtLayout {
    fn from(value: u64) -> Self {
        Self {
            indexes: [
                VADescriptor::L0.read(value),
                VADescriptor::L1.read(value),
                VADescriptor::L2.read(value),
                VADescriptor::L3.read(value),
            ],
            offset: VADescriptor::OFFSET.read(value),
        }
    }
}
