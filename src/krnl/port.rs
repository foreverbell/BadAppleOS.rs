#[derive(Clone, Copy)]
pub struct Port(u16);

impl Port {
  pub const fn new(port: u16) -> Port {
    Port(port)
  }

  pub const fn silbing(self) -> Port {
    Port(self.0 ^ 1)
  }
}

pub unsafe fn inb(port: Port) -> u8 {
  let ret: u8;
  asm!("inb %dx, %al" : "={ax}"(ret) : "{dx}"(port.0) :: "volatile");
  ret
}

pub unsafe fn inw(port: Port) -> u16 {
  let ret: u16;
  asm!("inw %dx, %ax" : "={ax}"(ret) : "{dx}"(port.0) :: "volatile");
  ret
}

pub unsafe fn outb(port: Port, val: u8) {
  asm!("outb %al, %dx" :: "{dx}"(port.0), "{al}"(val) :: "volatile");
}

pub unsafe fn outw(port: Port, val: u16) {
  asm!("outw %ax, %dx" :: "{dx}"(port.0), "{al}"(val) :: "volatile");
}

pub unsafe fn wait() {
  outb(Port(0x80), 0)
}
