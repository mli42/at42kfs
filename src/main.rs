#![no_std]
#![no_main]
#![no_builtins]
#![feature(lang_items)]
#![feature(alloc_error_handler)] // at the top of the file

#[macro_use]
extern crate alloc;

use core::panic::PanicInfo;

mod allocator;
mod gdt;
mod vga_buffer;

use vga_buffer::*;

use crate::allocator::init_heap;
use core::arch::asm;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    set_colors(Some(Color::Red), None);
    println!("{}", info);
    loop {}
}

#[allow(dead_code)]
extern "C" {
    fn stack_bottom();
    fn stack_top();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let gdt = gdt::GlobalDescriptorTable::init();
    gdt.install();

    init_heap();

    let v = 42;

    println!("GDT pointer: {:?}", gdt.get_gdt_pointer());

    println!("Stack bottom: 0x{:x}", stack_bottom as u32);
    println!("Stack top: 0x{:x}", stack_top as u32);
    println!("Our variable: {:p}", &v);

    println!("GDT dump:");
    gdt.print();

    println!("Stack dump:");
    hexdump(unsafe { (stack_top as *const u8).offset(-0x80) }, 0x80);

    loop {}
}
