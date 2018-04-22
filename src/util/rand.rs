use core::sync::atomic::{AtomicUsize, Ordering};

// HACK HACK
type AtomicU64 = AtomicUsize;

static NEXT: AtomicU64 = AtomicU64::new(1);

pub fn rand() -> u32 {
  loop {
    let cur = NEXT.load(Ordering::Relaxed);
    let next = cur * 214013 + 2531011;
    let old = NEXT.compare_and_swap(cur, next, Ordering::Relaxed);
    if old == cur {
      return ((next >> 16) & 0x7fff) as u32;
    }
  }
}

pub fn srand(seed: u32) {
  NEXT.store(seed as usize, Ordering::Relaxed);
}
