#![feature(alloc)]
#![feature(allocator_api)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(global_allocator)]
#![feature(integer_atomics)]
#![feature(lang_items)]
#![feature(offset_to)]
#![feature(ptr_internals)]
#![allow(dead_code)]
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
pub mod ba;

use ba::video;
use krnl::console;
use krnl::gdt;
use krnl::idt;
use krnl::irq;
use krnl::isr;
use krnl::power::{idle, sti};
use krnl::sys_time;
use krnl::timer;
use krnl::keyboard;
use mm::allocator::Allocator;
use spin::Mutex;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::instance();

lazy_static! {
  static ref BADAPPLE: Mutex<video::Video> = Mutex::new(video::Video::new());
}

fn play() {
  const FPS: u64 = 9;
  const TICK_PER_SEC: u64 = 18;

  BADAPPLE.lock().initialize();

  timer::TIMER.lock().add(
    TICK_PER_SEC,
    |timer: &mut timer::Timer,
     descriptor: timer::TimerDescriptor,
     count: u64| {
      if count < 3 {
        printf!("\rBadApple, {} second(s) to go", 3 - count);
        for _ in 0..count {
          printf!(".");
        }
      } else {
        timer.remove(descriptor);
        timer.add(
          TICK_PER_SEC / FPS,
          |timer: &mut timer::Timer,
           descriptor: timer::TimerDescriptor,
           _: u64| {
            let mut v = BADAPPLE.lock();
            if v.has_next() {
              v.next();
              printf!(" ({}%) ", v.progress());
            } else {
              timer.remove(descriptor);
              console::CONSOLE.lock().clear();
              printf!("Thank you for watching!\n");
              printf!("https://github.com/foreverbell/BadAppleOS.rs.\n");
              printf!("Press Enter to reboot.\n");
              keyboard::initialize();
              timer.add(
                1,
                |_: &mut timer::Timer, _: timer::TimerDescriptor, _: u64| {
                  printf!("\rCurrent system time = {}.", sys_time::get());
                },
              );
            }
          },
        );
      }
    },
  );
}

#[no_mangle]
pub extern "C" fn kinitialize() {
  console::initialize();

  printf!("Successfully landed to protected mode.\n");

  mm::init::initialize();
  gdt::initialize();
  idt::initialize();
  isr::initialize();
  irq::initialize();
  timer::initialize();

  unsafe {
    sti();
  }

  play();

  idle();
}
