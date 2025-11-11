#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    Terminal::new().write_string("[x] Bootloader stage 2 panicked! x_x");
    loop {}
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".start")]
pub extern "C" fn _start(arg1: u32) -> ! {

    print!("[k] starting kernel ...");
    println!(" arg1: {:x}", arg1);

    loop {}
}


pub struct Terminal {}

impl Terminal {
    pub fn new() -> Self {
        Terminal {}
    }

    pub fn write_byte(&self, byte: u8) {
        match byte {
            b'\n' => outb(0x3F8, '\n' as u8),
            byte => {
                outb(0x3F8, byte);
            }
        }
    }

    pub fn write_string(&self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}

impl fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
() => ($crate::print!("\n"));
($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    Terminal::new().write_fmt(args).unwrap();
}

pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags));
    }
}