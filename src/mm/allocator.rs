use alloc::heap::{Alloc, AllocErr, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

const KRNL_MEM_BEGIN: usize = 0xc0400000;
const KRNL_MEM_END: usize = 0xc0c00000;

// TODO TODO
// Explain why here we need AtomicUsize rather than Mutex.

pub struct Allocator(AtomicUsize);

impl Allocator {
  pub const fn instance() -> Allocator {
    Allocator(AtomicUsize::new(KRNL_MEM_BEGIN))
  }
}

fn align_by(n: usize, align: usize) -> usize {
  if !align.is_power_of_two() {
    return n;
  }
  n.saturating_add(align - 1) & !(align - 1)
}

unsafe impl<'a> Alloc for &'a Allocator {
  unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
    loop {
      let cur = self.0.load(Ordering::Relaxed);
      let aligned = align_by(cur, layout.align());
      let next = aligned.saturating_add(layout.size());

      if next > KRNL_MEM_END {
        return Err(AllocErr::Exhausted { request: layout });
      }

      let old = self.0.compare_and_swap(cur, next, Ordering::Relaxed);

      if old == cur {
        return Ok(aligned as *mut u8);
      }
    }
  }

  unsafe fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
    // Intentionally leak memory here.
  }
}
