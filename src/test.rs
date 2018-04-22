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
