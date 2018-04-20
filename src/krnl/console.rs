//
//  ------------------> y (80)
//  | > _             |
//  |     console     |
//  |                 |
//  x-----------------x
// (25)

const VIDEO_BASE: *mut Color = 0xb8000 as _;
const VIDEO_MAX_ROW: usize = 25;
const VIDEO_MAX_COLUMN: usize = 80;
const VIDEO_SIZE: usize = VIDEO_MAX_ROW * VIDEO_MAX_COLUMN;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ColorEnum {
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

impl ColorEnum {
  const DEFAULT_FORE: ColorEnum = ColorEnum::LightGrey;
  const DEFAULT_BACK: ColorEnum = ColorEnum::Black;
}

#[derive(Debug, Clone, Copy)]
pub struct Color(u8);

impl Color {
  const fn new(fore: ColorEnum, back: ColorEnum) -> Color {
    Color((back as u8) << 4 | (fore as u8))
  }

  const DEFAULT: Color =
    Color::new(ColorEnum::DEFAULT_FORE, ColorEnum::DEFAULT_BACK);
}
