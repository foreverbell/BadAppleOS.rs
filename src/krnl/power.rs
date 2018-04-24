use krnl::port;

pub unsafe fn halt() {
  asm!("hlt" :::: "volatile");
}

pub unsafe fn cli() {
  asm!("cli" :::: "volatile");
}

pub unsafe fn sti() {
  asm!("sti" :::: "volatile");
}

pub unsafe fn die() {
  cli();
  halt();
}

pub unsafe fn reboot() {
  port::outb(port::Port::new(0x64), 0xfe);
}

pub fn idle() {
  unsafe {
    loop {
      halt();
    }
  }
}
