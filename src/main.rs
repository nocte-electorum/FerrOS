#![allow(clippy::empty_loop, clippy::missing_safety_doc)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// VGA Buffer address: 0xb8000
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
	loop {}
}


#[panic_handler]
const fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
