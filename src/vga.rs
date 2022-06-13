#![allow(dead_code)]

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

///
/// An two-dimensional array representing the .
///
#[repr(transparent)]
struct Buffer {
    content: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

///
/// An enum of all colors supported by  
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

///
/// Some information used to draw a  to the screen.
///
/// Bits[0-3] -> background color
/// Bits[4-7] -> foreground color
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

///
/// Represents a character on-screen within  <br>
/// Consists of:
///     - An "ASCII" character code
///     - Some ColorCode data info
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

///
/// Writer struct to interact with a VGA Buffer.
///
pub struct Writer {
    buffer: &'static mut Buffer,
    color_code: ColorCode,
    cursor_position: (usize, usize),
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        color_code: ColorCode::new(Color::White, Color::Black),
        cursor_position: (0, 0) // x, y | col, row
    });
}

impl Writer {
    fn clear_row(&mut self, row: usize) -> () {
        for col in 0..BUFFER_WIDTH {
            self.buffer.content[row][col].write(ScreenChar {
                ascii_character: 0,
                color_code: ColorCode(0),
            })
        }
    }

    fn new_line(&mut self) {
        self.cursor_position.0 = 0;

        if (self.cursor_position.1 + 1) <= BUFFER_HEIGHT {
            self.cursor_position.1 += 1;
        } else { 
            for row in 0..BUFFER_HEIGHT - 1 {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.content[row + 1][col].read();
                    self.buffer.content[row][col].write(character);
                }
            }

            self.clear_row(BUFFER_HEIGHT - 1);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.cursor_position.0 >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.cursor_position.1;
                let col = self.cursor_position.0;

                let color_code = self.color_code;
                self.buffer.content[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.cursor_position.0 += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte), // Printable ASCII bytes | newline character
                _ => self.write_byte(0xfe),                   // Non-printables -> prints "■"
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
