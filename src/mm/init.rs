use core::ptr::Unique;
use krnl::cmos;
use volatile::Volatile;

bitflags! {
  struct EntryFlags: u32 {
    const PRESENT = 1 << 0;
    const WRITE = 1 << 1;
  }
}

type PageTable = Unique<[Volatile<u32>; 1024]>;

const PDE: PageTable = unsafe { Unique::new_unchecked(0x1000 as *mut _) };

const PTE: [PageTable; 4] = unsafe {
  [
    Unique::new_unchecked(0x2000 as *mut _),
    Unique::new_unchecked(0x3000 as *mut _),
    Unique::new_unchecked(0x4000 as *mut _),
    Unique::new_unchecked(0x5000 as *mut _),
  ]
};

const NVRAM_BASELO: u8 = 0x15;
const NVRAM_BASEHI: u8 = 0x16;
const NVRAM_EXTLO: u8 = 0x17;
const NVRAM_EXTHI: u8 = 0x18;
const NVRAM_EXT16LO: u8 = 0x34;
const NVRAM_EXT16HI: u8 = 0x35;

//
// Use CMOS calls to detect available base & extended memory.
// Measured in kilobytes.
//
fn detect() {
  let read = |register: u8| -> u32 { cmos::read(register) as u32 };

  let basemem = read(NVRAM_BASELO) | (read(NVRAM_BASEHI) << 8);
  let extmem = read(NVRAM_EXTLO) | (read(NVRAM_EXTHI) << 8);
  let ext16mem = (read(NVRAM_EXT16LO) | (read(NVRAM_EXT16HI) << 8)) << 6;
  let totalmem = if ext16mem != 0 {
    16 * 1024 + ext16mem
  } else if extmem != 0 {
    1 * 1024 + extmem
  } else {
    basemem
  };

  // We are not interested in this result.
  printf!(
    "[mm] Physical total = {}K, base = {}K, extended = {}K.\n",
    totalmem,
    basemem,
    totalmem - basemem
  );
}

pub fn initialize() {
  detect();
}
