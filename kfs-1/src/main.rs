#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)] // at the top of the file

#[macro_use]
extern crate alloc;

use core::panic::PanicInfo;

mod allocator;
mod vga_buffer;

use vga_buffer::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    set_colors(Some(Color::White), None);
    println!("Yolo, some numbers: {} {}", 4242, 1.455);

    set_colors(Some(Color::LightBlue), Some(Color::White));

    println!("Hello, your name is {}", "Motherfucking tate 😇");

    println!("This is it right ?");

    set_colors(Some(Color::LightGreen), Some(Color::Black));

    println!("Go out");

    let v = vec!["Yolo", "Totot"];

    println!("Vec {}", v.join(", "));

    loop {}
}
