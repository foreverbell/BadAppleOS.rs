#[derive(Clone, Copy)]
pub struct Port(u16);

pub unsafe fn inb(port: Port) -> u8 {
  let ret: u8;
  asm!("inb %dx, %al" : "={ax}"(ret) : "{dx}"(port.0) :: "volatile");
  return ret;
}

pub unsafe fn inw(port: Port) -> u16 {
  let ret: u16;
  asm!("inw %dx, %ax" : "={ax}"(ret) : "{dx}"(port.0) :: "volatile");
  return ret;
}

pub unsafe fn outb(port: Port, val: u8) {
  asm!("outb %al, %dx" :: "{dx}"(port.0), "{al}"(val) :: "volatile");
}

pub unsafe fn outw(port: Port, val: u16) {
  asm!("outw %ax, %dx" :: "{dx}"(port.0), "{al}"(val) :: "volatile");
}

pub unsafe fn wait() {
  asm!("outb %al, $0x80" :: "{al}"(0) :: "volatile");
}
