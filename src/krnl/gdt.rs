use spin::Mutex;
use core::mem;

extern "C" {
  fn gdt_flush(ptr: u32);
}

const MAX_ENTRIES: usize = 256;

pub const KRNL_CODE_SEL: u16 = 0x8;
pub const KRNL_DATA_SEL: u16 = 0x10;

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
struct GdtEntry {
  limit: u16,
  base_low: u16,
  base_middle: u8,
  access: u8,
  granularity: u8,
  base_high: u8,
}

#[repr(C, packed)]
#[derive(Default)]
struct GdtDescriptior {
  limit: u16,
  base: u32,
}

struct Gdt {
  entries: [GdtEntry; MAX_ENTRIES],
  descriptor: GdtDescriptior,
}

lazy_static! {
  static ref GDT: Mutex<Gdt> = Mutex::new(Gdt {
    entries: [GdtEntry::default(); MAX_ENTRIES],
    descriptor: GdtDescriptior::default(),
  });
}

impl Gdt {
  fn set_gate(
    self: &mut Gdt,
    index: usize,
    base: u32,
    limit: u16,
    access: u8,
    granularity: u8,
  ) {
    let entry = &mut self.entries[index];

    entry.base_low = (base & 0xffff) as u16;
    entry.base_middle = ((base >> 16) & 0xff) as u8;
    entry.base_high = ((base >> 24) & 0xff) as u8;
    entry.limit = limit;
    entry.access = access;
    entry.granularity = granularity;
  }

  fn flush(self: &mut Gdt) {
    self.descriptor.limit =
      (mem::size_of::<GdtEntry>() * MAX_ENTRIES - 1) as u16;
    self.descriptor.base = &self.entries as *const _ as u32;

    unsafe {
      gdt_flush(&self.descriptor as *const _ as u32);
    }
  }
}

pub fn initialize() {
  let mut gdt = GDT.lock();

  /* setup gdt gates. */
  gdt.set_gate(0, 0, 0x0, 0x0, 0x0); // null gdt entry
  gdt.set_gate(1, 0, 0xffff, 0x9a, 0xcf); // code segment
  gdt.set_gate(2, 0, 0xffff, 0x92, 0xcf); // data segment

  /* flush gdt. */
  gdt.flush();
}
