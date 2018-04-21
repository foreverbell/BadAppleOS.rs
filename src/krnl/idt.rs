use spin::Mutex;
use core::mem;

extern "C" {
  fn idt_flush(ptr: u32);
}

const MAX_ENTRIES: usize = 256;

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
struct IdtEntry {
  base_low: u16,
  sel: u16,
  always0: u8,
  flags: u8,
  base_high: u16,
}

#[repr(C, packed)]
#[derive(Default)]
struct IdtDescriptior {
  limit: u16,
  base: u32,
}

pub struct Idt {
  entries: [IdtEntry; MAX_ENTRIES],
  descriptor: IdtDescriptior,
}

lazy_static! {
  pub static ref IDT: Mutex<Idt> = Mutex::new(Idt {
    entries: [IdtEntry::default(); MAX_ENTRIES],
    descriptor: IdtDescriptior::default(),
  });
}

impl Idt {
  pub fn set_gate(
    self: &mut Idt,
    index: usize,
    base: u32,
    sel: u16,
    flags: u8,
  ) {
    let entry = &mut self.entries[index];

    entry.base_low = (base & 0xffff) as u16;
    entry.sel = sel;
    entry.always0 = 0;
    entry.flags = flags;
    entry.base_high = ((base >> 16) & 0xffff) as u16;
  }

  pub fn flush(self: &mut Idt) {
    self.descriptor.limit =
      (mem::size_of::<IdtEntry>() * MAX_ENTRIES - 1) as u16;
    self.descriptor.base = &self.entries as *const _ as u32;

    unsafe {
      idt_flush(&self.descriptor as *const _ as u32);
    }
  }
}

pub fn initialize() {
  IDT.lock().flush();
}
