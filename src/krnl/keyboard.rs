use krnl::irq;
use krnl::port::Port;
use krnl::port;
use krnl::power::reboot;

const PORT_PS2_DATA: Port = Port::new(0x60);
const SCAN_CODE_ENTER: u8 = 0x9c;

fn handler(_ctx: &irq::IrqContext) {
  unsafe {
    if port::inb(PORT_PS2_DATA) == SCAN_CODE_ENTER {
      printf!("Reboot.\n");
      reboot();
    }
  }
}

pub fn initialize() {
  // install and enable coresponding IRQ.
  irq::IRQ.lock().install(1, handler);
  unsafe {
    irq::Irq::enable(1);
  }
}
