#![feature(compiler_builtins_lib)]
#![feature(ptr_internals)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(asm)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![no_std]

extern crate ascii;
#[macro_use]
extern crate bitflags;
extern crate compiler_builtins;
#[macro_use]
extern crate lazy_static;
extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
mod util;
mod mm;
mod krnl;

use krnl::console;
use krnl::gdt;
use krnl::idt;

fn test() {
  console::CONSOLE.lock().setcolor(
    console::Color::White,
    console::Color::Blue,
    false,
  );
  printf!("Hello World!\n");
  console::CONSOLE.lock().setcolor(
    console::Color::LightGrey,
    console::Color::Black,
    false,
  );
  printf!("{}\n", krnl::sys_time::get());
}

#[no_mangle]
pub extern "C" fn kinitialize() {
  console::initialize();

  test();

  mm::init::initialize();
  gdt::initialize();
  idt::initialize();

  unsafe {
    asm!("hlt");
  }
}
