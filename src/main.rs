#![no_main]
#![no_std]

mod vga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("\n! === PANIC === !");
    println!("{}", _info);
    println!("! ============= !");

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome! {}", ":D");

    loop {}
}
