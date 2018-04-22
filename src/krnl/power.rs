pub unsafe fn halt() {
  asm!("hlt");
}

pub unsafe fn cli() {
  asm!("cli");
}

pub unsafe fn sti() {
  asm!("sti");
}

pub unsafe fn die() {
  cli();
  halt();
}

pub fn idle() {
  unsafe {
    loop {
      halt();
    }
  }
}
