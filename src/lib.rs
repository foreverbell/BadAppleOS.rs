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
pub mod util;
pub mod mm;
pub mod krnl;

use krnl::console;
use krnl::gdt;
use krnl::idt;
use krnl::irq;
use krnl::isr;
use mm::allocator::Allocator;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::instance();

fn test() {
  use console::Color::*;
  console::CONSOLE.lock().setcolor(White, Blue, false);
  printf!("Hello World!\n");
  console::CONSOLE.lock().setcolor(LightGrey, Black, false);
  printf!("{}\n", krnl::sys_time::get());
}

fn heap_test() {
  use alloc::boxed::Box;
  let heap_test = Box::new(42);
  printf!("box = {}\n", *heap_test);
}

fn keyboard_test() {
  unsafe { irq::Irq::enable(1); }
  printf!("Press any key to see an unhandled IRQ.\n");
}

fn int3_test() {
  unsafe {
    asm!("int $$3")
  }
}

#[no_mangle]
pub extern "C" fn kinitialize() {
  console::initialize();

  test();

  mm::init::initialize();
  gdt::initialize();
  idt::initialize();
  isr::initialize();
  irq::initialize();

  heap_test();
  keyboard_test();

  loop {
    unsafe {
      use krnl::power::{sti, halt};
      sti();
      halt();
    }
  }
}
