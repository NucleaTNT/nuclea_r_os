use crate::{
    gdt,
    output::vga::{Color, WRITER},
    pic, println,
};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(int_breakpoint_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(int_double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(pic::timer::int_timer_handler);
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(pic::keyboard::int_keyboard_handler);

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
