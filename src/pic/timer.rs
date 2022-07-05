use crate::print;

use crate::interrupts::{InterruptIndex, PICS};
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn int_timer_handler(_stack_frame: InterruptStackFrame) {
    print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
