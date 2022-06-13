#![allow(unused_imports)]
#![feature(custom_test_frameworks)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]
#![reexport_test_harness_main = "test_main"]
#![test_runner(nuclea_r_os::test_runner)]

use core::panic::PanicInfo;
use nuclea_r_os::println;
use nuclea_r_os::vga;

///
/// Kernel Entry Point
///
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    println!("Welcome! {}", ":D");

    nuclea_r_os::init();

    x86_64::instructions::interrupts::int3();

    println!("No crash!");

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    vga::WRITER.lock().color_code = vga::ColorCode {
        foreground_color: vga::Color::Red,
        background_color: vga::Color::Black,
    };

    println!("\n! === PANIC === !");
    println!(
        " Location: {}[{}:{}]",
        _info.location().unwrap().file(),
        _info.location().unwrap().line(),
        _info.location().unwrap().column()
    );
    println!(" Message: {:#?}", _info.message().unwrap());
    // println!(" Payload: {:#?}", _info.payload().downcast_ref::<&str>());
    println!("! ============= !");

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    nuclea_r_os::test_panic_handler(_info);
}

#[test_case]
fn test_grounding_assertion() {
    // Ensure all the laws of the universe are still in line.
    assert_eq!(1, 1);
}
