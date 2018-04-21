use krnl::port::Port;
use krnl::port;

const IRQ_VECTOR_OFFSET: u8 = 32;
const MAX_IRQ_ENTRIES: u32 = 16;

const PORT_MASTER_PIC_CMD: Port = Port::new(0x20);
const PORT_MASTER_PIC_DAT: Port = Port::new(0x21);

const PORT_SLAVE_PIC_CMD: Port = Port::new(0xa0);
const PORT_SLAVE_PIC_DAT: Port = Port::new(0xa1);

// FOR TIMER
// const PORT_PIT_CHANNEL0: Port = Port::new(0x40);
// const PORT_PIT_CHANNEL1: Port = Port::new(0x41);
// const PORT_PIT_CHANNEL2: Port = Port::new(0x42);
// const PORT_PIT_CMD: Port = Port::new(0x43);

struct IrqContext {
  gs: u32,
  fs: u32,
  es: u32,
  ds: u32,
  edi: u32,
  esi: u32,
  ebp: u32,
  esp: u32,
  ebx: u32,
  edx: u32,
  ecx: u32,
  eax: u32,
  irq_index: u32,
  eip: u32,
  cs: u32,
  eflags: u32,
}

pub fn initialize() {
  unsafe {
    /* remap IRQ to proper IDT entries (32 ~ 47) */
    port::outb(PORT_MASTER_PIC_CMD, 0x11);
    port::outb(PORT_SLAVE_PIC_CMD, 0x11);
    port::outb(PORT_MASTER_PIC_DAT, IRQ_VECTOR_OFFSET); // vector offset for master PIC is 32
    port::outb(PORT_SLAVE_PIC_DAT, IRQ_VECTOR_OFFSET + 8); // vector offset for slave PIC is 40
    port::outb(PORT_MASTER_PIC_DAT, 0x4); // tell master PIC that there is a slave PIC at IRQ2
    port::outb(PORT_SLAVE_PIC_DAT, 0x2); // tell slave PIC its cascade identity
    port::outb(PORT_MASTER_PIC_DAT, 0x1);
    port::outb(PORT_SLAVE_PIC_DAT, 0x1);

    /* disable all IRQs by default. */
    port::outb(PORT_MASTER_PIC_DAT, 0xff);
    port::outb(PORT_SLAVE_PIC_DAT, 0xff);
  }

  // TODO
}
