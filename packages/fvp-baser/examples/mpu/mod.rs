use core::ops::Range;

pub fn address_range_to_protected_region(range: Range<usize>) -> ProtectedRegion {
    // assuming FEAT_LPA is absent
    const MAX_ADDR: usize = 1 << 48;

    assert_eq!(
        0,
        range.start % 64,
        "start of address range not 64-byte aligned"
    );
    assert_eq!(
        0,
        range.end % 64,
        "end of address range not 64-byte aligned"
    );
    assert!(
        range.end > range.start,
        "address range is empty or inverted"
    );
    assert!(
        range.end < MAX_ADDR,
        "the address range is outside of the memory space that can be protected"
    );

    let base = range.start as u64 >> 6;
    // the PRBAR..=PRLAR range is *inclusive*
    // the value in PRLAR will be `<< 6` and then concatenated with `0x3F`
    // given that `range` does NOT include the end address but it's 64-byte
    // aligned we have to subtract one to represent the same range in PRLAR
    let limit = (range.end as u64 - 1) >> 6;

    ProtectedRegion { base, limit }
}

/// A protected region
pub struct ProtectedRegion {
    /// PRBAR value
    pub base: u64,
    /// PRLAR value
    pub limit: u64,
}
