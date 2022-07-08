#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
	Black = 0x0,
	Blue = 0x1,
	Green = 0x2,
	Cyan = 0x3,
	Red = 0x4,
	Magenta = 0x5,
	Brown = 0x6,
	LightGray = 0x7,
	DarkGray = 0x8,
	LightBlue = 0x9,
	LightGreen = 0xa,
	LightCyan = 0xb,
	LightRed = 0xc,
	Pink = 0xd,
	Yellow = 0xe,
	White = 0xf,
}


const MAX_HEIGHT: i32 = 25;
const MAX_WIDTH: i32 = 80;

pub struct Writer {
	column: i32,
	row: i32
}

// Constants
impl Writer {
	const ADDR: *mut u8 = 0xb8000 as *mut u8;

	pub fn new() -> Self {
		Self {
			column: 1,
			row: 1
		}
	}
}