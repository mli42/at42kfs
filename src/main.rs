#![no_std]
#![no_main]
#![no_builtins]

use core::panic::PanicInfo;

mod vga_buffer;

use vga_buffer::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    set_colors(Some(Color::Red), None);
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    set_colors(Some(Color::White), None);
    println!("Yolo, some numbers: {} {}", 4242, 1.455);

    set_colors(Some(Color::LightBlue), Some(Color::White));

    println!("Hello, your name is {}", "Hi ! 😇");

    println!("This is it right ?");

    set_colors(Some(Color::LightGreen), Some(Color::Black));

    println!("Go out");

    loop {}
}
