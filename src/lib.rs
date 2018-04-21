#![feature(compiler_builtins_lib)]
#![feature(ptr_internals)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(asm)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![no_std]

extern crate ascii;
extern crate compiler_builtins;
extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod util;
mod krnl;

use krnl::console::CONSOLE;
use krnl::gdt;
use krnl::idt;

#[no_mangle]
pub extern "C" fn kinitialize() {
  CONSOLE.lock().clear();

  let hello = b"Hello World!";
  let color_byte = 0x1f; // white foreground, blue background

  let mut hello_colored = [color_byte; 24];
  for (i, char_byte) in hello.into_iter().enumerate() {
    hello_colored[i * 2] = *char_byte;
  }

  // write `Hello World!` to the center of the VGA text buffer
  let buffer_ptr = 0xb8000 as *mut _;
  unsafe { *buffer_ptr = hello_colored };

  CONSOLE.lock().putch(b'\n');
  CONSOLE.lock().putch(b'a');

  printf!(" {:?}\n", Some(666));

  printf!("{}\n", krnl::sys_time::get());

  gdt::initialize();
  idt::initialize();

  unsafe {
    asm!("hlt");
  }
}
