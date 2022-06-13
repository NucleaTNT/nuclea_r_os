#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(panic_info_message)]
#![no_std]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::test_runner)]

pub mod qemu;
pub mod serial;
pub mod vga;

use crate::qemu::{exit_qemu, QEMUExitCode};
use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}... ", core::any::type_name::<T>());
        self();
        serial_println!("[OK]");
    }
}

///
/// `cargo test` entry point.
///
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    test_panic_handler(_info);
}

pub fn test_panic_handler(_info: &PanicInfo) -> ! {
    serial_println!("[FAILED]\n\n! === Error Info === !");
    serial_println!(
        " Location: {}[{}:{}]",
        _info.location().unwrap().file(),
        _info.location().unwrap().line(),
        _info.location().unwrap().column()
    );
    serial_println!(" Message: {:#?}", _info.message().unwrap());
    // serial_println!(" Payload: {:#?}", _info.payload().downcast_ref::<&str>());
    serial_println!("! === END === !\n");

    exit_qemu(QEMUExitCode::Failed);

    loop {}
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("\nRunning {} test(s)...", tests.len());

    for test in tests {
        test.run();
    }

    serial_println!("Tests complete! Exiting.\n");
    exit_qemu(QEMUExitCode::Success);
}