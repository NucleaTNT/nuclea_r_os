#![allow(dead_code)]

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const TAB_WIDTH: usize = 4;

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
/// Bits \[0-3] -> background color <br>
/// Bits \[4-7] -> foreground color
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorCode {
    pub foreground_color: Color,
    pub background_color: Color,
}

impl ColorCode {
    pub fn get_value(&self, foreground: Option<Color>, background: Option<Color>) -> u8 {
        (background.unwrap_or(self.background_color) as u8) << 4
            | (foreground.unwrap_or(self.foreground_color) as u8)
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
struct ScreenChar {
    ascii_character: u8,
    color_code: u8,
}

///
/// Writer struct to interact with a VGA Buffer.
///
pub struct Writer {
    buffer: &'static mut Buffer,
    pub color_code: ColorCode,
    pub cursor_position: (usize, usize),
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        color_code: ColorCode { foreground_color: Color::White, background_color: Color::Black },
        cursor_position: (0, 0) // x, y | col, row
    });
}

impl Writer {
    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.content[row][col].write(ScreenChar {
                ascii_character: 0,
                color_code: self.color_code.get_value(
                    Some(self.color_code.background_color),
                    Some(self.color_code.background_color),
                ),
            })
        }
    }

    fn new_line(&mut self) {
        self.cursor_position.0 = 0;

        if (self.cursor_position.1 + 1) < BUFFER_HEIGHT {
            self.cursor_position.1 += 1;
        } else {
            for row in 0..BUFFER_HEIGHT - 1 {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.content[row + 1][col].read();
                    self.buffer.content[row][col].write(character);
                }
            }

            self.clear_row(BUFFER_HEIGHT - 1);
            self.cursor_position.1 = BUFFER_HEIGHT - 1;
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\t' => {
                for _ in 0..TAB_WIDTH {
                    self.write_byte(b' ');
                }
            }
            _ => {
                if self.cursor_position.0 >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.cursor_position.1;
                let col = self.cursor_position.0;

                self.buffer.content[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code.get_value(None, None),
                });
                self.cursor_position.0 += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' | b'\t' => self.write_byte(byte), // Printable ASCII bytes | newline character
                _ => self.write_byte(0xfe), // Non-printables -> prints "■"
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

#[test_case]
fn test_println_basic() {
    println!("<test_println_basic output>");
}

#[test_case]
fn test_println_vga_overflow() {
    for _ in 0..100 {
        println!("<test_println_vga_overflow output>");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Test string that fits on a single line.";
    let row = WRITER.lock().cursor_position.1;

    println!("{}", s);

    for (i, chr) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.content[row - 1][i].read();
        assert_eq!(char::from(screen_char.ascii_character), chr);
    }
}
