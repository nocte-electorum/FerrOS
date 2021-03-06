use spin::{ Mutex, Lazy };

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

struct Writer {
	column: i32,
	row: i32
}

// Constants
impl Writer {
	const ADDR: *mut u8 = 0xb8000 as *mut u8;

	pub const fn new() -> Self {
		Self {
			column: 1,
			row: 1
		}
	}
}

// Methods
impl Writer {
	pub fn write_byte(&mut self, color: Color, byte: u8) {
		if byte == b'\n' {
			self.new_line();
		} else {
			if self.row > MAX_HEIGHT && self.column > MAX_WIDTH {
				return;
			}

			let offset: i32 = (self.row - 1) * 80 + (self.column - 1);

			unsafe {
				*(Self::ADDR.add(offset as usize * 2_usize)) = byte;
				*(Self::ADDR.add(offset as usize * 2_usize + 1)) = color as u8;
			}

			if self.column > MAX_WIDTH {
				self.column = 1;
				self.row += 1;
			} else {
				self.column += 1;
			}
		}
	}

	pub fn new_line(&mut self) {
		self.row += 1;
		self.column = 1;
	}
}


pub struct VGABuffer {
	writer: Writer,
	color: Color
}

// Constants
impl VGABuffer {
	pub const fn new(color: Color) -> Self {
		let writer: Writer = Writer::new();
		Self {
			writer,
			color
		}
	}
}

// Methods
impl VGABuffer {
	pub fn write_bytes(&mut self, bytes: &[u8], color_override: Option<Color>) {
		let color: Color = color_override.unwrap_or(self.color);
		for byte in bytes {
			self.writer.write_byte(color, *byte);
		}
	}

	pub fn write_char(&mut self, c: char, color_override: Option<Color>) {
		let color: Color = color_override.unwrap_or(self.color);
		self.writer.write_byte(color, c as u8);
	}

	/// Convenience function
	pub fn write_str(&mut self, to_write: &str, color_override: Option<Color>) {
		self.write_bytes(to_write.as_bytes(), color_override);
	}
}



pub static BUFFER: Lazy<Mutex<VGABuffer>> = Lazy::new(|| {
	Mutex::new(VGABuffer::new(Color::White))
});

macro_rules! print {
	($s:literal) => {
		BUFFER.lock().write_str($s, None);
	};
}
pub(crate) use print;

macro_rules! println {
	() => {
		print!("\n");
	};
	($s:literal) => {
		print!($s);
		print!("\n");
	}
}
pub(crate) use println;