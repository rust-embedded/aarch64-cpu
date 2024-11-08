use super::asm::barrier::{dsb, isb, NSH, SY};
use crate::{
    asm::cache::{dc, ic, CISW, CIVAC, CSW, CVAC, IALLU, ISW, IVAC},
    irq::IRQ,
    mmu::{MMRegion, MMSegment},
    registers::{Readable, Writeable, CCSIDR_EL1, CLIDR_EL1, CSSELR_EL1},
};

pub struct ICache {}
pub struct DCache {}

pub const ICACHE: ICache = ICache {};
pub const DCACHE: DCache = DCache {};

pub enum FlushMode {
    INV,
    CLEAN,
    INVCLEAN,
}

pub enum AddressMode {
    WAYSET,
    VIRTUAL,
}

impl DCache {
    fn get_flush_func(&self, flush_mode: FlushMode, address_mode: AddressMode) -> impl Fn(u64) {
        match address_mode {
            AddressMode::WAYSET => match flush_mode {
                FlushMode::INV => move |addr: u64| dc(ISW, addr),
                FlushMode::CLEAN => move |addr: u64| dc(CSW, addr),
                FlushMode::INVCLEAN => move |addr: u64| dc(CISW, addr),
            },
            AddressMode::VIRTUAL => match flush_mode {
                FlushMode::INV => move |addr: u64| dc(IVAC, addr),
                FlushMode::CLEAN => move |addr: u64| dc(CVAC, addr),
                FlushMode::INVCLEAN => move |addr: u64| dc(CIVAC, addr),
            },
        }
    }

    #[inline(always)]
    pub fn flush(&self, addr: u64, mode: FlushMode) {
        self.get_flush_func(mode, AddressMode::VIRTUAL)(addr);
        dsb(SY);
        isb(SY);
    }

    pub fn flush_region(&self, virt: MMSegment, mode: FlushMode) {
        MMRegion::new(virt, CCSIDR_EL1.read(CCSIDR_EL1::LineSize) + 4)
            .into_iter()
            .for_each(self.get_flush_func(mode, AddressMode::VIRTUAL));
        dsb(SY);
        isb(SY);
    }

    #[inline(always)]
    pub fn flush_all(&self, mode: FlushMode) {
        dsb(SY);
        let loc = CLIDR_EL1.read(CLIDR_EL1::LoC) * 2;
        let clidr = CLIDR_EL1.get();
        let _flush = self.get_flush_func(mode, AddressMode::WAYSET);
        let mut irq = IRQ::new();

        if loc != 0 {
            for level in (0..loc).step_by(2) {
                if (clidr >> (level + level >> 1)) & 0b111 < 2 {
                    continue;
                }

                irq.save_and_disable();
                CSSELR_EL1.set(level);
                isb(SY);
                let ls = CCSIDR_EL1.read(CCSIDR_EL1::LineSize) + 4; // Notice that linesize needs taken special care of since CCSIDR_EL1
                                                                    // only offers log2(Words of Cache) instead of granularity of bytes
                let m = CCSIDR_EL1.read(CCSIDR_EL1::AssociativityWithoutCCIDX);
                let n = CCSIDR_EL1.read(CCSIDR_EL1::NumSetsWithoutCCIDX);
                let k = m.leading_zeros();

                irq.restore_and_enable();

                (0..m + 1)
                    .flat_map(move |i| (0..n + 1).map(move |j| (i << k) | level | (j << ls)))
                    .for_each(&_flush);
            }
        }

        CSSELR_EL1.set(0);
        dsb(SY);
        isb(SY);
    }
}

impl ICache {
    pub fn flush_all(&self) {
        ic(IALLU);
        dsb(NSH);
        isb(SY);
    }
}
