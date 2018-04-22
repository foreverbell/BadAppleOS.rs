use ba::decompressor::{decompress, Decompressed};
use util::rand::{rand, srand};

extern "C" {
  static _binary_build_vdata_bin_start: u8;
  static _binary_build_vdata_bin_end: u8;
}

pub fn test() {
  let vdata_start: *const u8 =
    unsafe { &_binary_build_vdata_bin_start as *const _ };
  let vdata_end: *const u8 =
    unsafe { &_binary_build_vdata_bin_end as *const _ };

  decompress(vdata_start, vdata_end);
}
