#![feature(custom_test_frameworks)]
#![no_main]
#![no_std]
#![test_runner(nuclea_r_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use nuclea_r_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    nuclea_r_os::test_panic_handler(_info);
}

#[test_case]
fn test_println() {
    println!("<test_println output>");
}
