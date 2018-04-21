#![feature(allocator_api)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(global_allocator)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(ptr_internals)]
#![allow(dead_code)]
#![feature(alloc)]
#![allow(non_snake_case)]
#![no_std]

extern crate alloc;
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
use mm::allocator::Allocator;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::instance();

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

fn heap_test() {
  use alloc::boxed::Box;
  let heap_test = Box::new(42);
  printf!("box = {}\n", *heap_test);
}

#[no_mangle]
pub extern "C" fn kinitialize() {
  console::initialize();

  test();

  mm::init::initialize();
  gdt::initialize();
  idt::initialize();

  heap_test();

  unsafe {
    asm!("hlt");
  }
}
