pub unsafe fn halt() {
  asm!("hlt");
}

pub unsafe fn cli() {
  asm!("cli");
}

pub unsafe fn die() {
  cli();
  halt();
}
