#![feature(abi_x86_interrupt)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use nuclea_r_os::{
    gdt::DOUBLE_FAULT_IST_INDEX,
    qemu::{exit_qemu, QEMUExitCode},
    serial_print, serial_println,
};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("\nstack_overflow::test_stack_overflow... ");

    nuclea_r_os::gdt::init_gdt();
    init_test_idt();

    test_stack_overflow();

    panic!("[FAILED] - Execution continued after stack overflow.");
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    nuclea_r_os::test_panic_handler(_info);
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _err_code: u64,
) -> ! {
    serial_println!("[OK]");
    serial_println!("Test complete! Exiting.\n");

    exit_qemu(QEMUExitCode::Success);

    loop {}
}

#[allow(unconditional_recursion)]
fn test_stack_overflow() {
    test_stack_overflow();
    volatile::Volatile::new(0).read(); // Prevent tail recursion optimizations
}
