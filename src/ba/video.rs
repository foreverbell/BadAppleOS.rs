use alloc::vec::Vec;
use ba::decompressor::decompress;
use ba::stream::Stream;
use krnl::console;

extern "C" {
  static _binary_build_vdata_bin_start: u8;
  static _binary_build_vdata_bin_end: u8;
}

pub struct Video {
  n_frames: usize,
  cur_frame: usize,
  frames: Vec<console::ConsoleBuf>,
}

// artify the frame to emphasize boundary.
pub fn artify(frame: &mut console::ConsoleBuf) {
  const DXY: [[isize; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];
  const DOT_CHARS: [char; 4] = [',', '.', '\'', '`'];
  const LINE_CHARS: [char; 4] = ['v', '^', '\\', '/'];

  let within = |x: isize, y: isize| -> bool {
    return x >= 0 && x < console::MAX_ROW as isize && y >= 0
      && y < console::MAX_COLUMN as isize;
  };
  let get = |frame: &console::ConsoleBuf, x: isize, y: isize| -> char {
    frame[x as usize][y as usize].ch as char
  };
  let set = |frame: &mut console::ConsoleBuf, x: isize, y: isize, ch: char| {
    frame[x as usize][y as usize].ch = ch as u8;
  };

  for x in 0..console::MAX_ROW as isize {
    for y in 0..console::MAX_COLUMN as isize {
      if get(frame, x, y) == ' ' {
        continue;
      }
      let mut dir: Option<usize> = None;
      let mut n_empty: usize = 0;
      for d in 0..4 {
        if within(x + DXY[d][0], y + DXY[d][1])
          && get(frame, x + DXY[d][0], y + DXY[d][1]) == ' '
        {
          dir = Some(d);
          n_empty += 1;
        }
      }
      if let Some(d) = dir {
        let mut use_line = true;
        if n_empty > 2 {
          use_line = false;
        }
        if within(x + DXY[(3 - d) ^ 1][0], y + DXY[(3 - d) ^ 1][1])
          && get(frame, x + DXY[(3 - d) ^ 1][0], y + DXY[(3 - d) ^ 1][1]) == ' '
        {
          use_line = false;
        }
        if within(x + DXY[3 - d][0], y + DXY[3 - d][1])
          && get(frame, x + DXY[3 - d][0], y + DXY[3 - d][1]) == ' '
        {
          use_line = false;
        }
        let new_char = if use_line {
          LINE_CHARS[d]
        } else {
          DOT_CHARS[d]
        };
        set(frame, x, y, new_char);
      }
    }
  }
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

    printf!("[video] Decompressing data.\n");

    match decompress(vdata_start, vdata_end) {
      Some(decompressed) => {
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
          artify(&mut self.frames[f]);
        }

        printf!("[video] Data loaded.\n");
      },
      None => printf!("[video] Corrupted Data.\n"),
    }
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
