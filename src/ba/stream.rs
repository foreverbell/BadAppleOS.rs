pub struct Stream<'a> {
  data: &'a [u8],
  seek: usize,
  shift: usize,
}

impl<'a> Stream<'a> {
  pub fn new(data: &'a [u8]) -> Stream {
    Stream {
      data: data,
      seek: 0,
      shift: 0,
    }
  }

  pub fn has_next(&self) -> bool {
    self.seek < self.data.len()
  }

  pub fn next_byte(&mut self) -> u8 {
    assert!(self.has_next());

    let bit = (self.data[self.seek] >> self.shift) & 1;
    self.shift += 1;
    if self.shift == 8 {
      self.shift = 0;
      self.seek += 1;
    }
    bit
  }

  pub fn remain(&self) -> usize {
    (self.data.len() - self.seek) * 8 - self.shift
  }
}
