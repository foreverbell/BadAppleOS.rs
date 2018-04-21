use core::ptr::Unique;
use krnl::cmos;
use volatile::Volatile;

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

bitflags! {
  struct PageEntryFlags: u32 {
    const PRESENT = 1 << 0;
    const WRITE = 1 << 1;
  }
}

const TABLE_ENTRIES: usize = 1024;
type PageTable = Unique<[Volatile<u32>; TABLE_ENTRIES]>;

pub fn initialize() {
  detect();

  const PDE: PageTable = unsafe { Unique::new_unchecked(0x1000 as *mut _) };
  const PTES: [PageTable; 4] = unsafe {
    [
      Unique::new_unchecked(0x2000 as *mut _),
      Unique::new_unchecked(0x3000 as *mut _),
      Unique::new_unchecked(0x4000 as *mut _),
      Unique::new_unchecked(0x5000 as *mut _),
    ]
  };
  let ENTRY_FLAGS: u32 =
    (PageEntryFlags::PRESENT | PageEntryFlags::WRITE).bits();

  // clear all PDE entries.
  for i in 0..TABLE_ENTRIES {
    unsafe {
      PDE.as_mut()[i].write(0);
    }
  }

  let setup = move |index: usize,
                    PTE: &mut PageTable,
                    phys: u32,
                    from: usize,
                    to: usize| {
    unsafe {
      PDE.as_mut()[index].write((PTE.as_ptr() as u32) | ENTRY_FLAGS);

      for i in 0..TABLE_ENTRIES {
        PTE.as_mut()[i].write(0);
        if i >= from && i < to {
          PTE.as_mut()[i].write((((i as u32) << 12) + phys) | ENTRY_FLAGS);
        }
      }
    }
  };

  // set lower 1M memory as identity paging.
  setup(0, &mut PTES[0], 0x0, 0, 256);

  // 0xc0000000 ~ 0xc00f0000 for kernel (physical 0x0 ~ 0xf0000), 960K.
  setup(768, &mut PTES[1], 0x0, 0, 240);

  // leave a memory hole here as guard pages.

  // 0xc0400000 ~ 0xc0c00000 for memory pool (physical 0x100000 ~ 0x900000), 8M.
  setup(769, &mut PTES[2], 0x100000, 0, 1024);
  setup(770, &mut PTES[3], 0x500000, 0, 1024);

  unsafe {
    asm!("mov $0, %cr3" :: "r" (PDE.as_ptr() as u32) : "memory" : "volatile");
  }
}
