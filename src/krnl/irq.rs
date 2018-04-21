use krnl::gdt;
use krnl::idt;
use krnl::port::Port;
use krnl::port;
use spin::Mutex;

extern "C" {
  fn irq_handler0();
  fn irq_handler1();
  fn irq_handler2();
  fn irq_handler3();
  fn irq_handler4();
  fn irq_handler5();
  fn irq_handler6();
  fn irq_handler7();
  fn irq_handler8();
  fn irq_handler9();
  fn irq_handler10();
  fn irq_handler11();
  fn irq_handler12();
  fn irq_handler13();
  fn irq_handler14();
  fn irq_handler15();
}

const IRQ_VECTOR_OFFSET: u8 = 32;
const MAX_IRQ_ENTRIES: usize = 16;

const PORT_MASTER_PIC_CMD: Port = Port::new(0x20);
const PORT_MASTER_PIC_DAT: Port = Port::new(0x21);

const PORT_SLAVE_PIC_CMD: Port = Port::new(0xa0);
const PORT_SLAVE_PIC_DAT: Port = Port::new(0xa1);

pub struct IrqContext {
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

type IrqFn = fn(&IrqContext) -> ();

pub struct Irq {
  fns: ([Option<IrqFn>; MAX_IRQ_ENTRIES]),
}

pub static IRQ: Mutex<Irq> = Mutex::new(Irq {
  fns: [None; MAX_IRQ_ENTRIES],
});

impl Irq {
  // install an IRQ handler.
  pub fn install(&mut self, index: usize, handler: IrqFn) {
    if self.fns[index].is_some() {
      printf!(
        "[irq::install] IRQ{} handler already exists, overwritten.\n",
        index
      );
    }
    self.fns[index] = Some(handler)
  }

  // uninstall an IRQ handler.
  pub fn uninstall(&mut self, index: usize) {
    if self.fns[index].is_none() {
      printf!("[irq::uninstall] IRQ{} handler not exists.\n", index);
    }
    self.fns[index] = None
  }

  unsafe fn set_mask(index: i32, set: bool) {
    let (port, n_index) = if index < 8 {
      (PORT_MASTER_PIC_DAT, index)
    } else {
      (PORT_SLAVE_PIC_DAT, index - 8)
    };
    let value = port::inb(port);
    if set {
      port::outb(port, value | (1 << n_index))
    } else {
      port::outb(port, value & !(1 << n_index))
    }
  }

  // disable an IRQ by setting mask.
  pub unsafe fn disable(index: i32) {
    Self::set_mask(index, true)
  }

  // enable an IRQ by clearing mask.
  pub unsafe fn enable(index: i32) {
    Self::set_mask(index, false)
  }
}

#[no_mangle]
pub extern "C" fn irq_dispatcher(ctx: IrqContext) {
  let irq_index = ctx.irq_index as usize;
  let irq_fn = IRQ.lock().fns[irq_index];

  if irq_fn.is_none() {
    printf!("[IRQ dispatcher] Unhandled IRQ {}.\n", irq_index);
  } else {
    irq_fn.unwrap()(&ctx);
  }

  // send an EOI (end of interrupt) to indicate that we are done.
  unsafe {
    if irq_index >= 8 {
      port::outb(PORT_SLAVE_PIC_CMD, 0x20);
    }
    port::outb(PORT_MASTER_PIC_CMD, 0x20);
  }
}

pub fn initialize() {
  unsafe {
    // remap IRQ to proper IDT entries (32 ~ 47).
    port::outb(PORT_MASTER_PIC_CMD, 0x11);
    port::outb(PORT_SLAVE_PIC_CMD, 0x11);
    port::outb(PORT_MASTER_PIC_DAT, IRQ_VECTOR_OFFSET); // vector offset for master PIC is 32
    port::outb(PORT_SLAVE_PIC_DAT, IRQ_VECTOR_OFFSET + 8); // vector offset for slave PIC is 40
    port::outb(PORT_MASTER_PIC_DAT, 0x4); // tell master PIC that there is a slave PIC at IRQ2
    port::outb(PORT_SLAVE_PIC_DAT, 0x2); // tell slave PIC its cascade identity
    port::outb(PORT_MASTER_PIC_DAT, 0x1);
    port::outb(PORT_SLAVE_PIC_DAT, 0x1);

    // disable all IRQs by default.
    port::outb(PORT_MASTER_PIC_DAT, 0xff);
    port::outb(PORT_SLAVE_PIC_DAT, 0xff);
  }

  // initialize IRQ to correct entries in the IDT.
  let mut idt = idt::IDT.lock();

  macro_rules! set_irq {
    ($id:expr, $fun:ident) => {
      idt.set_gate(
        ($id + IRQ_VECTOR_OFFSET) as usize,
        $fun as *const () as u32,
        gdt::KRNL_CODE_SEL,
        0x8e
      );
    };
  }

  set_irq!(0, irq_handler0);
  set_irq!(1, irq_handler1);
  set_irq!(2, irq_handler2);
  set_irq!(3, irq_handler3);
  set_irq!(4, irq_handler4);
  set_irq!(5, irq_handler5);
  set_irq!(6, irq_handler6);
  set_irq!(7, irq_handler7);
  set_irq!(8, irq_handler8);
  set_irq!(9, irq_handler9);
  set_irq!(10, irq_handler10);
  set_irq!(11, irq_handler11);
  set_irq!(12, irq_handler12);
  set_irq!(13, irq_handler13);
  set_irq!(14, irq_handler14);
  set_irq!(15, irq_handler15);
}
