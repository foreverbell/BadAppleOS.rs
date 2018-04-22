use krnl::irq;
use krnl::port::Port;
use krnl::port;
use spin::Mutex;

const PORT_PIT_CHANNEL0: Port = Port::new(0x40);
const PORT_PIT_CHANNEL1: Port = Port::new(0x41);
const PORT_PIT_CHANNEL2: Port = Port::new(0x42);
const PORT_PIT_CMD: Port = Port::new(0x43);

const MAX_ENTRIES: usize = 16;

pub struct TimerDescriptor(usize); // essentially index in timer entries array.

type TimerFn = fn(&mut Timer, TimerDescriptor, u64) -> ();

#[derive(Copy, Clone)]
struct TimerEntry {
  interval: u64,
  trigger_count: u64,
  count_down: u64,
  f: TimerFn,
}

pub struct Timer {
  entries: ([Option<TimerEntry>; MAX_ENTRIES]),
  ticks: u64,
}

pub static TIMER: Mutex<Timer> = Mutex::new(Timer {
  entries: [None; MAX_ENTRIES],
  ticks: 0,
});

impl Timer {
  pub fn ticks(&self) -> u64 {
    self.ticks
  }

  pub fn add(&mut self, interval: u64, f: TimerFn) -> Option<TimerDescriptor> {
    if interval == 0 {
      return None;
    }
    for i in 0..MAX_ENTRIES {
      if self.entries[i].is_some() {
        continue;
      }
      self.entries[i] = Some(TimerEntry {
        interval: interval,
        trigger_count: 0,
        count_down: interval,
        f: f,
      });
      return Some(TimerDescriptor(i));
    }
    None
  }

  pub fn remove(&mut self, d: TimerDescriptor) -> bool {
    let index = d.0;
    if self.entries[index].is_none() {
      return false;
    }
    self.entries[index] = None;
    true
  }
}

fn handler(_ctx: &irq::IrqContext) {
  match TIMER.try_lock() {
    None => {
      // Timer lock is held by somebody else before this interrupt happens,
      // give up.
      return;
    },
    Some(mut timer) => {
      timer.ticks += 1;

      for i in 0..MAX_ENTRIES {
        if timer.entries[i].is_none() {
          continue;
        }
        let mut call = false;
        {
          let mut e = timer.entries[i].as_mut().unwrap();
          e.count_down -= 1;
          if e.count_down == 0 {
            e.trigger_count += 1;
            e.count_down = e.interval;
            call = true;
          }
        }
        if call {
          let e = timer.entries[i].unwrap();
          let f = e.f;
          let tick = e.trigger_count;
          f(&mut timer, TimerDescriptor(i), tick);
        }
      }
    },
  }
}

pub fn initialize() {
  // default tick rate, 18 ticks = 1 second.
  unsafe {
    port::outb(PORT_PIT_CMD, 0x36);
    port::outb(PORT_PIT_CHANNEL0, 0);
    port::outb(PORT_PIT_CHANNEL0, 0);
  }

  // install and enable coresponding IRQ.
  irq::IRQ.lock().install(0, handler);
  unsafe {
    irq::Irq::enable(0);
  }
}
