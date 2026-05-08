use core::{
    fmt,
    sync::atomic::{self, AtomicUsize},
};

pub struct Nesting;

impl Nesting {
    pub fn increase() -> NestingGuard {
        NESTING.fetch_add(1, atomic::Ordering::Relaxed);
        NestingGuard { _inner: () }
    }
}

pub struct NestingGuard {
    _inner: (),
}

static NESTING: AtomicUsize = AtomicUsize::new(0);

impl fmt::Display for Nesting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nesting = NESTING.load(atomic::Ordering::Relaxed);
        for _ in 0..nesting {
            f.write_str("  ")?;
        }
        Ok(())
    }
}

impl Drop for NestingGuard {
    fn drop(&mut self) {
        NESTING.fetch_sub(1, atomic::Ordering::Relaxed);
    }
}
