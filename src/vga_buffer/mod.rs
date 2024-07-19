#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

use core::mem::transmute_copy;

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }

    pub fn get_foreground(self) -> Color {
        let color4bit = (self.0 as u8) & 0b1111;
        unsafe {
            return transmute_copy(&color4bit);
            // Justification: "color4bit" is forcefully in the range [0;15] and the enum Color accept u8 values in the range [0;15].
        }
    }

    pub fn get_background(self) -> Color {
        let color4bit = (self.0 as u8) >> 4;
        unsafe {
            return transmute_copy(&color4bit);
            // Justification: "color4bit" is forcefully in the range [0;15] and the enum Color accept u8 values in the range [0;15].
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;

#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub column_position: usize,
    pub row_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    pub fn set_colors(&mut self, foreground: Option<Color>, background: Option<Color>) {
        self.color_code = ColorCode::new(
            foreground.unwrap_or(self.color_code.get_foreground()),
            background.unwrap_or(self.color_code.get_background()),
        )
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in self.column_position..BUFFER_WIDTH {
            self.buffer.chars[BUFFER_HEIGHT - 2][col].write(blank);
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        row_position: 0,
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use crate::interrupts::without_interrupts;
    use core::fmt::Write;

    without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn set_colors(foreground: Option<Color>, background: Option<Color>) {
    WRITER.lock().set_colors(foreground, background)
}

pub fn hexdump(view: *const u8, size: usize) {
    for i in (0..size).step_by(16) {
        let ptr = unsafe { view.offset(i as isize) };
        let max_j = if size - i >= 16 { 16 } else { size - i };

        // Display address
        print!("0x{:08x}: ", (ptr as usize));

        // Display hex code
        for j in 0..16 {
            if j < max_j {
                print!("{:02x} ", unsafe { *ptr.offset(j as isize) });
            } else {
                print!("   ");
            }
        }

        // Display printable characters
        for j in 0..max_j {
            let byte = unsafe { *ptr.offset(j as isize) };

            print!(
                "{}",
                match byte {
                    0x20..=0x7e => byte as char,
                    _ => '.',
                }
            )
        }

        println!();
    }
    println!();
}
