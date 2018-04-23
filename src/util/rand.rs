use core::sync::atomic::{AtomicU32, Ordering};

static NEXT: AtomicU32 = AtomicU32::new(1);

pub fn rand() -> u32 {
  loop {
    let cur = NEXT.load(Ordering::Relaxed);
    let next = cur * 214013 + 2531011;
    let old = NEXT.compare_and_swap(cur, next, Ordering::Relaxed);
    if old == cur {
      return (next >> 16) & 0x7fff;
    }
  }
}

pub fn srand(seed: u32) {
  NEXT.store(seed, Ordering::Relaxed);
}
