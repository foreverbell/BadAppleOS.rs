use krnl::power::die;
use core::fmt;

#[lang = "eh_personality"]
#[no_mangle]
#[allow(private_no_mangle_fns)]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
#[allow(private_no_mangle_fns)]
pub extern "C" fn panic_fmt(
  _: fmt::Arguments,
  file: &'static str,
  line: u32,
  column: u32,
) -> () {
  printf!("panic at {} L{}:{}.\n", file, line, column);
  unsafe {
    die();
  }
}
