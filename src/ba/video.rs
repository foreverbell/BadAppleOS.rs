use alloc::vec::Vec;
use ba::decompressor::decompress;
use ba::stream::Stream;
use krnl::console;
use util::rand::{rand, srand};

extern "C" {
  static _binary_build_vdata_bin_start: u8;
  static _binary_build_vdata_bin_end: u8;
}

pub struct Video {
  n_frames: usize,
  cur_frame: usize,
  frames: Vec<console::ConsoleBuf>,
}

impl Video {
  pub fn new() -> Video {
    Video {
      n_frames: 0,
      cur_frame: 0,
      frames: Vec::new(),
    }
  }

  pub fn initialize(&mut self) {
    let vdata_start: *const u8 =
      unsafe { &_binary_build_vdata_bin_start as *const _ };
    let vdata_end: *const u8 =
      unsafe { &_binary_build_vdata_bin_end as *const _ };

    printf!("[video] decompressing data.\n");

    let decompressed = decompress(vdata_start, vdata_end);

    self.n_frames = decompressed.n_frames;
    self.frames = Vec::new();

    let mut reader = Stream::new(decompressed.buf.as_slice());

    self.frames.resize(
      self.n_frames,
      [[Default::default(); console::MAX_COLUMN]; console::MAX_ROW],
    );
    for f in 0..self.n_frames {
      for row in 0..console::MAX_ROW {
        for col in 0..console::MAX_COLUMN {
          let ch = if reader.next_byte() == 0 { '%' } else { ' ' };
          self.frames[f][row][col] =
            console::ScreenChar::new(ch as u8, Default::default());
        }
      }
    }

    printf!("[video] data loaded.\n");
  }

  pub fn progress(&self) -> usize {
    (self.cur_frame + 1) * 100 / self.n_frames
  }

  pub fn has_next(&self) -> bool {
    self.cur_frame < self.n_frames
  }

  pub fn next(&mut self) {
    console::CONSOLE.lock().bkcpy(&self.frames[self.cur_frame]);
    self.cur_frame += 1;
  }
}
