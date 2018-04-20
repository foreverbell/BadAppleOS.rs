use core::ptr::Unique;
use krnl::port::*;
use spin::Mutex;
use volatile::Volatile;
use ascii::AsciiChar;

//
//  ------------------> y (80)
//  | > _             |
//  |     console     |
//  |                 |
//  x-----------------x
// (25)

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
struct Attribute(u8);

impl Attribute {
  const fn new(fore: Color, back: Color) -> Attribute {
    Attribute((back as u8) << 4 | (fore as u8))
  }

  const DEFAULT: Attribute =
    Attribute::new(Color::DEFAULT_FORE, Color::DEFAULT_BACK);
}

#[derive(Clone, Copy)]
#[repr(C)]
struct ScreenChar {
  char: u8,
  attrib: Attribute,
}

struct VideoBuffer {
  chars:
    [[Volatile<ScreenChar>; VideoBuffer::MAX_COLUMN]; VideoBuffer::MAX_ROW],
}

impl VideoBuffer {
  const BUFFER: *mut VideoBuffer = 0xb8000 as _;
  const MAX_ROW: usize = 25;
  const MAX_COLUMN: usize = 80;
  const SIZE: usize = VideoBuffer::MAX_ROW * VideoBuffer::MAX_COLUMN;
}

struct Cursor {
  vport: Port,
  x: usize, // row
  y: usize, // column
  show: bool,
}

impl Cursor {
  fn go(vport: Port, x: usize, y: usize) {
    let offset: usize = x * VideoBuffer::MAX_COLUMN + y;
    let vport2: Port = vport.silbing();

    unsafe {
      outb(vport, 14);
      outb(vport2, (offset >> 8) as u8);
      outb(vport, 15);
      outb(vport2, offset as u8);
    }
  }

  fn new(vport: Port) -> Cursor {
    let mut offset: usize;
    let vport2: Port = vport.silbing();

    unsafe {
      outb(vport, 0xe);
      offset = (inb(vport2) as usize) << 8;
      outb(vport, 0xf);
      offset += inb(vport2) as usize;
    }

    let x: usize = offset / VideoBuffer::MAX_COLUMN;
    let y: usize = offset % VideoBuffer::MAX_COLUMN;

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
    Cursor::go(self.vport, VideoBuffer::MAX_ROW, 0)
  }
}

pub struct Console {
  buffer: Unique<VideoBuffer>,
  cursor: Cursor,
  attrib: Attribute,
}

lazy_static! {
  pub static ref CONSOLE: Mutex<Console> = {
    let vport = unsafe { Port::new(*(0x463 as *mut u16)) };
    let mut console = Console {
      buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
      cursor: Cursor::new(vport),
      attrib: Attribute::DEFAULT,
    };

    console.setcolor(Color::DEFAULT_FORE, Color::DEFAULT_BACK, true);

    Mutex::new(console)
  };
}

impl Console {
  fn buffer_ref(&self) -> &VideoBuffer {
    unsafe { self.buffer.as_ref() }
  }

  fn buffer_mut(&mut self) -> &mut VideoBuffer {
    unsafe { self.buffer.as_mut() }
  }

  fn scroll(&mut self) {
    let space = ScreenChar {
      char: AsciiChar::Space as u8,
      attrib: self.attrib,
    };

    for row in 0..VideoBuffer::MAX_ROW - 1 {
      for col in 0..VideoBuffer::MAX_COLUMN {
        let old = self.buffer_ref().chars[row + 1][col].read();
        self.buffer_mut().chars[row][col].write(old);
      }
    }
    for col in 0..VideoBuffer::MAX_COLUMN {
      self.buffer_mut().chars[VideoBuffer::MAX_ROW - 1][col].write(space);
    }
  }

  pub fn hide_cursor(&mut self) {
    self.cursor.hide()
  }

  pub fn setcolor(&mut self, fore: Color, back: Color, reset: bool) {
    let attrib: Attribute = Attribute::new(fore, back);
    if reset {
      for row in 0..VideoBuffer::MAX_ROW {
        for col in 0..VideoBuffer::MAX_COLUMN {
          let char = self.buffer_ref().chars[row][col].read().char;
          self.buffer_mut().chars[row][col].write(ScreenChar { char, attrib });
        }
      }
    }
    self.attrib = attrib
  }

  pub fn clear(&mut self) {
    let space = ScreenChar {
      char: AsciiChar::Space as u8,
      attrib: self.attrib,
    };

    for row in 0..VideoBuffer::MAX_ROW {
      for col in 0..VideoBuffer::MAX_COLUMN {
        self.buffer_mut().chars[row][col].write(space);
      }
    }
    self.cursor.x = 0;
    self.cursor.y = 0;
  }

  pub fn putch(&mut self, char: u8) {
    match AsciiChar::from(char).unwrap() {
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
        self.buffer_mut().chars[row][col].write(ScreenChar { char, attrib });
        self.cursor.y += 1;
      },
    }
    if self.cursor.y >= VideoBuffer::MAX_COLUMN {
      self.cursor.y = 0;
      self.cursor.x += 1;
    }
    if self.cursor.x >= VideoBuffer::MAX_ROW {
      self.scroll();
      self.cursor.x -= 1;
    }
  }
}
