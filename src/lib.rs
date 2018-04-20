#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![no_std]

extern crate compiler_builtins;
extern crate rlibc;
extern crate volatile;
extern crate spin;

#[no_mangle]
pub extern fn kinitialize() {
  let hello = b"Hello World!";
  let color_byte = 0x1f; // white foreground, blue background

  let mut hello_colored = [color_byte; 24];
  for (i, char_byte) in hello.into_iter().enumerate() {
      hello_colored[i*2] = *char_byte;
  }

  // write `Hello World!` to the center of the VGA text buffer
  let buffer_ptr = 0xb8000 as *mut _;
  unsafe { *buffer_ptr = hello_colored };

  loop { }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() { }

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! {
  loop { }
}
