#![no_std]
#![no_main]

use core::panic::PanicInfo;

// System entry point. 
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
// Panic Hanlder required for no_std builds
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

