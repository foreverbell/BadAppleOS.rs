use krnl::power::die;
use krnl::idt;
use krnl::gdt;

extern "C" {
  fn isr_handler0();
  fn isr_handler1();
  fn isr_handler2();
  fn isr_handler3();
  fn isr_handler4();
  fn isr_handler5();
  fn isr_handler6();
  fn isr_handler7();
  fn isr_handler8();
  fn isr_handler9();
  fn isr_handler10();
  fn isr_handler11();
  fn isr_handler12();
  fn isr_handler13();
  fn isr_handler14();
  fn isr_handler15();
  fn isr_handler16();
  fn isr_handler17();
  fn isr_handler18();
}

const EXCEPTION_MESSAGE: [&'static str; 19] = [
  "Division By Zero Exception",            // fault
  "Debug Exception",                       // fault/trap
  "Non Maskable Interrupt Exception",      // interrupt
  "Breakpoint Exception",                  // trap
  "Into Detected Overflow Exception",      // trap
  "Out of Bounds Exception",               // fault
  "Invalid Opcode Exception",              // fault
  "No Coprocessor Exception",              // fault
  "Double Fault Exception",                // abort
  "Coprocessor Segment Overrun Exception", // fault
  "Bad TSS Exception",                     // fault
  "Segment Not Present Exception",         // fault
  "Stack Fault Exception",                 // fault
  "General Protection Fault Exception",    // fault
  "Page Fault Exception",                  // fault
  "Unknown Interrupt Exception",           // unknown
  "Coprocessor Fault Exception",           // fault
  "Alignment Check Exception",             // fault
  "Machine Check Exception",               // abort
];

pub struct IsrContext {
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
  isr_index: u32,
  error_code: u32,
  eip: u32,
  cs: u32,
  eflags: u32,
}

#[no_mangle]
pub extern "C" fn isr_dispatcher(ctx: IsrContext) {
  printf!(
    "Exception = {}, with error code = {}.\n",
    EXCEPTION_MESSAGE[ctx.isr_index as usize],
    ctx.error_code
  );
  printf!("Registers:\n");
  printf!(
    "\tds  0x{:08x}\tes  0x{:08x}\tfs  0x{:08x}\tgs  0x{:08x}\n",
    ctx.ds,
    ctx.es,
    ctx.fs,
    ctx.gs
  );
  printf!(
    "\teax 0x{:08x}\tebx 0x{:08x}\tecx 0x{:08x}\tedx 0x{:08x}\n",
    ctx.eax,
    ctx.ebx,
    ctx.ecx,
    ctx.edx
  );
  printf!(
    "\tesp 0x{:08x}\tebp 0x{:08x}\tesi 0x{:08x}\tedi 0x{:08x}\n",
    ctx.esp,
    ctx.ebp,
    ctx.esi,
    ctx.edi
  );
  printf!(
    "\teip 0x{:08x}\tcs  0x{:08x}\teflags 0x{:08x}\n",
    ctx.eip,
    ctx.cs,
    ctx.eflags
  );

  printf!("System halted.\n");
  unsafe {
    die();
  }
}

pub fn initialize() {
  let mut idt = idt::IDT.lock();

  macro_rules! set_isr {
    ($id:expr, $fun:ident) => {
      idt.set_gate($id, $fun as *const () as u32, gdt::KRNL_CODE_SEL, 0x8e);
    };
  }

  set_isr!(0, isr_handler0);
  set_isr!(1, isr_handler1);
  set_isr!(2, isr_handler2);
  set_isr!(3, isr_handler3);
  set_isr!(4, isr_handler4);
  set_isr!(5, isr_handler5);
  set_isr!(6, isr_handler6);
  set_isr!(7, isr_handler7);
  set_isr!(8, isr_handler8);
  set_isr!(9, isr_handler9);
  set_isr!(10, isr_handler10);
  set_isr!(11, isr_handler11);
  set_isr!(12, isr_handler12);
  set_isr!(13, isr_handler13);
  set_isr!(14, isr_handler14);
  set_isr!(15, isr_handler15);
  set_isr!(16, isr_handler16);
  set_isr!(17, isr_handler17);
  set_isr!(18, isr_handler18);
}
