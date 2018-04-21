use krnl::port::{inb, outb, Port};

const PORT_CMOS_CMD: Port = Port::new(0x70);
const PORT_CMOS_DAT: Port = Port::new(0x71);

pub fn read(register: u8) -> u8 {
  let result: u8;
  unsafe {
    outb(PORT_CMOS_CMD, register);
    result = inb(PORT_CMOS_DAT);
  }
  result
}
