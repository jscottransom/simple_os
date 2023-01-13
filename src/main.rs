#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(simple_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use simple_os::println;

// System entry point.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    // Conditional Call. Will only show when you call c
    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    simple_os::test_panic_handler(info)
}
