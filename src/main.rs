#![no_std]
#![no_main]
#![no_builtins]
#![feature(alloc_error_handler)]
#![feature(abi_x86_interrupt)]

use crate::allocator::init_heap;
use crate::interrupts::init_idt;
use core::arch::asm;
use core::panic::PanicInfo;
use vga_buffer::*;

mod allocator;
mod asm;
mod gdt;
mod interrupts;
mod io;
mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    set_colors(Some(Color::Red), None);
    println!("{}", info);
    loop {
        halt!();
    }
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

    unsafe {
        init_idt();
        // idt::PICS.lock().initialize();
        asm!("sti");
    };

    let _ = init_heap();

    let v = 42;

    println!("GDT pointer: {:?}", gdt.get_gdt_pointer());

    println!("Stack bottom: 0x{:x}", stack_bottom as u32);
    println!("Stack top: 0x{:x}", stack_top as u32);
    println!("Our variable: {:p}", &v);

    println!("GDT dump:");
    gdt.print();

    println!("Stack dump:");
    hexdump(unsafe { (stack_top as *const u8).offset(-0x80) }, 0x80);

    unsafe {
        asm!("int 3");
    }

    loop {
        halt!();
    }
}
