#![feature(allocator_api)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(global_allocator)]
#![feature(lang_items)]
#![feature(offset_to)]
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
use mm::allocator::Allocator;
use spin::Mutex;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::instance();

mod test {
  use krnl::console;
  use krnl::irq;
  use krnl::timer;

  pub fn console_test() {
    use console::Color::*;
    use krnl::sys_time;

    console::CONSOLE.lock().setcolor(White, Blue, false);
    printf!("Hello World!\n");
    console::CONSOLE.lock().setcolor(LightGrey, Black, false);
    printf!("{}\n", sys_time::get());
  }

  pub fn heap_test() {
    use alloc::boxed::Box;
    let heap_test = Box::new(42);
    printf!("box = {}\n", *heap_test);
  }

  pub fn keyboard_test() {
    unsafe {
      irq::Irq::enable(1);
    }
    printf!("Press any key to see an unhandled IRQ.\n");
  }

  pub fn int3_test() {
    unsafe { asm!("int $$3") }
  }

  pub fn timer_test() {
    let mut timer = timer::TIMER.lock();
    timer.add(
      5,
      |t: &mut timer::Timer, _td: timer::TimerDescriptor, tick: u64| {
        printf!(
          "5 ticks has passed, {}, triggered {} times.\n",
          t.ticks(),
          tick
        );
      },
    );
  }
}

lazy_static! {
  pub static ref BADAPPLE: Mutex<video::Video> = Mutex::new(video::Video::new());
}

fn play() {
  const FPS: u64 = 9;
  const TIMER_TICK_PER_SECOND: u64 = 18;

  BADAPPLE.lock().initialize();

  timer::TIMER.lock().add(
    TIMER_TICK_PER_SECOND,
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
          TIMER_TICK_PER_SECOND / FPS,
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

  // test::console_test();

  mm::init::initialize();
  gdt::initialize();
  idt::initialize();
  isr::initialize();
  irq::initialize();
  timer::initialize();

  // test::heap_test();
  // test::keyboard_test();

  // test::timer_test();
  play();

  unsafe {
    sti();
  }

  idle();
}
