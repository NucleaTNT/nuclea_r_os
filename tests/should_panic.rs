#![no_main]
#![no_std]

use core::panic::PanicInfo;
use nuclea_r_os::{
    qemu::{exit_qemu, QEMUExitCode},
    serial_println, serial_print,
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_should_panic();

    serial_println!("[FAILED] - Test failed to panic.\n");
    exit_qemu(QEMUExitCode::Failed);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[OK]");

    serial_println!("Tests complete! Exiting.\n");
    exit_qemu(QEMUExitCode::Success);

    loop {}
}

fn test_should_panic() {
    serial_print!("\nshould_panic::test_should_panic... ");
    assert_eq!(0, 1);
}