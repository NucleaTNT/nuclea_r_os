use crate::{println, vga::{Color, WRITER}};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(int_breakpoint_handler);
    
        idt
    };
}

pub fn idt_init() {
    IDT.load();
}

extern "x86-interrupt" fn int_breakpoint_handler(stack_frame: InterruptStackFrame) {
    let fg = WRITER.lock().color_code.foreground_color;
    WRITER.lock().color_code.foreground_color = Color::Yellow;
    println!("\nException Raised: BREAKPOINT\n{:#?}\n", stack_frame);
    WRITER.lock().color_code.foreground_color = fg;
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}