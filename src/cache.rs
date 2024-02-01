
use core::arch::global_asm;
use super::asm::barrier::{NONE, isb, dsb, NSH};
use super::asm::cache::{ic, IALLUIS};
use crate::asm::cache::{dc, CVAC, IVAC, CIVAC};
use crate::mmu::address::Segment;

global_asm!(include_str!("./asm/cache.S"));

mod sealed {
    use crate::mmu::address::Segment;
    pub trait Cache {
        fn clean(&self, seg: Segment);
        fn inv(&self, seg: Segment);
        fn inv_all(&self);
        fn inv_clean(&self, seg: Segment);
    }
}

extern "C" {
    pub fn dcache_invall();
}

pub struct ICache {}
pub struct DCache {}

pub const ICACHE: ICache = ICache {};
pub const DCACHE: DCache = DCache {};

impl sealed::Cache for DCache {

    #[inline(always)]
    fn clean(&self, seg: Segment) {
        dc(IVAC, seg)
    }

    #[inline(always)]
    fn inv(&self, seg: Segment) {
        dc(CVAC, seg)
    }

    #[inline(always)]
    fn inv_all(&self) {
        unsafe {
            dcache_invall();
        }
    }

    #[inline(always)]
    fn inv_clean(&self, seg: Segment) {
        dc(CIVAC, seg);
    }
}

impl sealed::Cache for ICache{
    #[inline(always)]
    fn clean(&self, seg: Segment){
    }

    #[inline(always)]
    fn inv(&self, seg:Segment){

    }

    #[inline(always)]
    fn inv_all(&self){
        ic(IALLUIS);
    }

    #[inline(always)]
    fn inv_clean(&self, seg: Segment) {

    }

}

pub enum CacheFlush {
    INV,
    CLEAN,
    INVCLEAN,
}

pub fn flush_cache_area(_cache: impl sealed::Cache, seg: Segment, mode: CacheFlush) {
    match mode {
        CacheFlush::INV => _cache.inv(seg),
        CacheFlush::INVCLEAN => _cache.inv_clean(seg),
        CacheFlush::CLEAN => _cache.clean(seg),
    }
    dsb(NSH);
    isb(NONE);
}

pub fn flush_cache_all(_cache: impl sealed::Cache) {
    _cache.inv_all();
    dsb(NSH);
    isb(NONE);
}
