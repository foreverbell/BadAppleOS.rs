use ascii::AsciiChar;
use core::ptr::Unique;
use krnl::port;
use krnl::port::Port;
use spin::Mutex;
use volatile::Volatile;

//
//  ------------------> y (80)
//  | > _             |
//  |     console     |
//  |                 |
//  x-----------------x
// (25)

pub const MAX_ROW: usize = 25;
pub const MAX_COLUMN: usize = 80;
pub const SIZE: usize = MAX_ROW * MAX_COLUMN;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Color {
  Black = 0,
  Blue = 1,
  Green = 2,
  Cyan = 3,
  Red = 4,
  Magenta = 5,
  Brown = 6,
  LightGrey = 7,
  DarkGrey = 8,
  LightBlue = 9,
  LightGreen = 10,
  LightCyan = 11,
  LightRed = 12,
  Pink = 13,
  Yellow = 14,
  White = 15,
}

impl Color {
  const DEFAULT_FORE: Color = Color::LightGrey;
  const DEFAULT_BACK: Color = Color::Black;
}

// An attribute is composition of two colors - the foreground and background.
#[derive(Clone, Copy)]
pub struct Attribute(u8);

impl Attribute {
  pub const fn new(fore: Color, back: Color) -> Attribute {
    Attribute((back as u8) << 4 | (fore as u8))
  }
}

impl Default for Attribute {
  fn default() -> Attribute {
    Attribute::new(Color::DEFAULT_FORE, Color::DEFAULT_BACK)
  }
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct ScreenChar {
  ch: u8,
  attrib: Attribute,
}

impl ScreenChar {
  pub const fn new(ch: u8, attrib: Attribute) -> ScreenChar {
    ScreenChar { ch, attrib }
  }
}

pub type ConsoleBuf = [[ScreenChar; MAX_COLUMN]; MAX_ROW];
pub type ConsoleBufVolatile = [[Volatile<ScreenChar>; MAX_COLUMN]; MAX_ROW];

struct Cursor {
  vport: Port,
  x: usize, // row
  y: usize, // column
  show: bool,
}

impl Cursor {
  fn go(vport: Port, x: usize, y: usize) {
    let offset: usize = x * MAX_COLUMN + y;
    let vport2: Port = vport.silbing();

    unsafe {
      port::outb(vport, 14);
      port::outb(vport2, (offset >> 8) as u8);
      port::outb(vport, 15);
      port::outb(vport2, offset as u8);
    }
  }

  fn new(vport: Port) -> Cursor {
    let mut offset: usize;
    let vport2: Port = vport.silbing();

    unsafe {
      port::outb(vport, 0xe);
      offset = (port::inb(vport2) as usize) << 8;
      port::outb(vport, 0xf);
      offset += port::inb(vport2) as usize;
    }

    let x: usize = offset / MAX_COLUMN;
    let y: usize = offset % MAX_COLUMN;

    Cursor {
      vport: vport,
      x: x,
      y: y,
      show: true,
    }
  }

  fn push(&self) {
    if self.show {
      Cursor::go(self.vport, self.x, self.y)
    }
  }

  fn hide(&mut self) {
    self.show = false;
    Cursor::go(self.vport, MAX_ROW, 0)
  }
}

pub struct Console {
  buf: Unique<ConsoleBufVolatile>,
  cursor: Cursor,
  attrib: Attribute,
}

lazy_static! {
  pub static ref CONSOLE: Mutex<Console> = {
    let vport = unsafe { Port::new(*(0x463 as *mut u16)) };
    let console = Console {
      buf: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
      cursor: Cursor::new(vport),
      attrib: Default::default(),
    };

    Mutex::new(console)
  };
}

impl Console {
  fn buf_ref(&self) -> &ConsoleBufVolatile {
    unsafe { self.buf.as_ref() }
  }

  fn buf_mut(&mut self) -> &mut ConsoleBufVolatile {
    unsafe { self.buf.as_mut() }
  }

  fn scroll(&mut self) {
    let space = ScreenChar {
      ch: AsciiChar::Space as u8,
      attrib: self.attrib,
    };

    for row in 0..MAX_ROW - 1 {
      for col in 0..MAX_COLUMN {
        let old = self.buf_ref()[row + 1][col].read();
        self.buf_mut()[row][col].write(old);
      }
    }
    for col in 0..MAX_COLUMN {
      self.buf_mut()[MAX_ROW - 1][col].write(space);
    }
  }

  pub fn hide_cursor(&mut self) {
    self.cursor.hide()
  }

  pub fn setcolor(&mut self, fore: Color, back: Color, reset: bool) {
    let attrib: Attribute = Attribute::new(fore, back);
    if reset {
      for row in 0..MAX_ROW {
        for col in 0..MAX_COLUMN {
          let ch = self.buf_ref()[row][col].read().ch;
          self.buf_mut()[row][col].write(ScreenChar { ch, attrib });
        }
      }
    }
    self.attrib = attrib
  }

  pub fn clear(&mut self) {
    let space = ScreenChar {
      ch: AsciiChar::Space as u8,
      attrib: self.attrib,
    };

    for row in 0..MAX_ROW {
      for col in 0..MAX_COLUMN {
        self.buf_mut()[row][col].write(space);
      }
    }
    self.cursor.x = 0;
    self.cursor.y = 0;
    self.cursor.push();

    unsafe { port::wait() }
  }

  pub fn bkcpy(&mut self, buf: &ConsoleBuf) {
    for row in 0..MAX_ROW {
      for col in 0..MAX_COLUMN {
        self.buf_mut()[row][col].write(buf[row][col]);
      }
    }
    self.cursor.x = 0;
    self.cursor.y = 0;
    self.cursor.push();

    unsafe { port::wait() }
  }

  pub fn putch(&mut self, ch: u8) {
    match AsciiChar::from(ch).unwrap() {
      AsciiChar::BackSpace => {
        if self.cursor.y != 0 {
          self.cursor.y -= 1;
        }
      },
      AsciiChar::Tab => {
        self.cursor.y = (self.cursor.y + 4) & !3;
      },
      AsciiChar::CarriageReturn => {
        self.cursor.y = 0;
      },
      AsciiChar::LineFeed => {
        self.cursor.y = 0;
        self.cursor.x += 1;
      },
      _ => {
        let row = self.cursor.x;
        let col = self.cursor.y;
        let attrib = self.attrib;
        self.buf_mut()[row][col].write(ScreenChar { ch, attrib });
        self.cursor.y += 1;
      },
    }
    if self.cursor.y >= MAX_COLUMN {
      self.cursor.y = 0;
      self.cursor.x += 1;
    }
    if self.cursor.x >= MAX_ROW {
      self.scroll();
      self.cursor.x -= 1;
    }
    self.cursor.push()
  }
}

pub fn initialize() {
  let mut console = CONSOLE.lock();

  console.setcolor(Color::DEFAULT_FORE, Color::DEFAULT_BACK, true);
  console.clear();
}
