use krnl::port::Port;
use krnl::port;
use krnl::irq;

const PORT_PIT_CHANNEL0: Port = Port::new(0x40);
const PORT_PIT_CHANNEL1: Port = Port::new(0x41);
const PORT_PIT_CHANNEL2: Port = Port::new(0x42);
const PORT_PIT_CMD: Port = Port::new(0x43);

fn handler(_ctx: &irq::IrqContext) {
  printf!("One tick has passed.\n");
}

pub fn initialize() {
  // default tick rate, 18 ticks = 1 second.
  unsafe {
    port::outb(PORT_PIT_CMD, 0x36);
    port::outb(PORT_PIT_CHANNEL0, 0);
    port::outb(PORT_PIT_CHANNEL0, 0);
  }

  // install and enable coresponding IRQ.
  irq::IRQ.lock().install(0, handler);
  unsafe {
    irq::Irq::enable(0);
  }
}
