#![no_std]
#![no_main]

use core::panic::PanicInfo;

// VGA Buffer address: 0xb8000
#[no_mangle]
pub extern "C" fn main() {
	loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
