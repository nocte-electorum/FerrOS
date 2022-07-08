#![allow(clippy::empty_loop, clippy::missing_safety_doc, dead_code, unused)]
#![no_std]
#![no_main]

mod vga_buffer;
use vga_buffer::{ Color, VGABuffer };
use core::panic::PanicInfo;

const HELLO: &[u8] = b"Hello World!";
// VGA Buffer address: 0xb8000
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
	// let addr: *mut u8 = 0xb8000 as *mut u8;

	// for (i, byte) in HELLO.iter().enumerate() {
	// 	*(addr.add(i * 2)) = *byte;
	// 	*(addr.add(i * 2 + 1)) = Color::Red as u8;
	// }
	let mut buffer: VGABuffer = VGABuffer::new(Color::White);
	buffer.write_str("0000000000", None);
	buffer.write_str("0000000000", None);
	buffer.write_str("0000000000", None);
	buffer.write_str("0000000000", None);
	buffer.write_str("0000000000", None);
	buffer.write_str("0000000000", None);
	buffer.write_str("0000000000", None);
	buffer.write_str("0000000000", None);
	buffer.write_str("0000000000", None);
	loop {}
}


#[panic_handler]
const fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
