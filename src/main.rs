#![feature(custom_test_frameworks)]
#![no_main]
#![no_std]

mod vga;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome! {}", ":D");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("\n! === PANIC === !");
    println!("{}", _info);
    println!("! ============= !");

    loop {}
}
