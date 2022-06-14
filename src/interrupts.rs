use crate::{
    gdt, println,
    vga::{Color, WRITER},
};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(int_breakpoint_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(int_double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn int_breakpoint_handler(_stack_frame: InterruptStackFrame) {
    let fg = WRITER.lock().color_code.foreground_color;
    WRITER.lock().color_code.foreground_color = Color::Yellow;
    println!("\nException Raised: BREAKPOINT\n{:#?}\n", _stack_frame);
    WRITER.lock().color_code.foreground_color = fg;
}

extern "x86-interrupt" fn int_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _err_code: u64,
) -> ! {
    panic!("\n\tException Raised: DOUBLE FAULT\n\t{:#?}", _stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
