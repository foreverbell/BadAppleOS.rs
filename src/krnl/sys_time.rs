use krnl::cmos::read;
use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SysTime {
  year: u32,
  month: u32,
  day: u32,
  hour: u32,
  minute: u32,
  second: u32,
}

impl fmt::Display for SysTime {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}/{:02}/{:02} {:02}:{:02}:{:02}",
      self.year, self.month, self.day, self.hour, self.minute, self.second
    )
  }
}

fn try_get() -> SysTime {
  while (read(0xa) & 0x80) != 0 {
    unsafe {
      asm!("nop");
    }
  }
  SysTime {
    second: read(0x0) as u32,
    minute: read(0x2) as u32,
    hour: read(0x4) as u32,
    day: read(0x7) as u32,
    month: read(0x8) as u32,
    year: read(0x9) as u32,
  }
}

fn bcd2bin(v: u32) -> u32 {
  (v & 0xf) + (v >> 4) * 10
}

pub fn get() -> SysTime {
  let mut cur_time: SysTime;
  let mut last_time: SysTime = try_get();

  // avoid CMOS is updating time when fetching it.
  loop {
    cur_time = try_get();
    if last_time == cur_time {
      break;
    }
    last_time = cur_time;
  }

  let status_B = read(0xb) as u32;
  let hour_h8 = cur_time.hour & 0x80;

  cur_time.hour &= !0x80;

  if (!status_B & 0x4) != 0 {
    cur_time = SysTime {
      second: bcd2bin(cur_time.second),
      minute: bcd2bin(cur_time.minute),
      hour: bcd2bin(cur_time.hour),
      day: bcd2bin(cur_time.day),
      month: bcd2bin(cur_time.month),
      year: bcd2bin(cur_time.year),
    }
  }

  // convert 12 hour to 24 hour if necessary.
  if (!status_B & 0x2) != 0 && hour_h8 != 0 {
    cur_time.hour = (cur_time.hour + 12) % 24;
  }

  // 2-digit to full 4-digit.
  cur_time.year += 2000;

  cur_time
}
