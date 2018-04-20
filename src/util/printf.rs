use core::fmt;
use core::fmt::Write;
use krnl::console::{Console, CONSOLE};

impl fmt::Write for Console {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for byte in s.bytes() {
      self.putch(byte)
    }
    Ok(())
  }
}

pub fn printf(args: fmt::Arguments) {
  CONSOLE.lock().write_fmt(args).unwrap();
}

macro_rules! printf {
  ($($arg:tt)*) => ({
    util::printf::printf(format_args!($($arg)*));
  });
}
