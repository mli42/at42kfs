#![no_std]
#![no_main]
#![no_builtins]
#![feature(abi_x86_interrupt)]

use core::arch::asm;
use core::panic::PanicInfo;
use vga_buffer::*;

mod cli;
mod gdt;
mod interrupts;
mod io;
mod keyboard;
mod panic;
mod utils;
mod vga_buffer;

#[allow(dead_code)]
extern "C" {
    fn stack_bottom();
    fn stack_top();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let gdt = gdt::GlobalDescriptorTable::init();
    gdt.install();

    interrupts::init_idt();
    interrupts::pic8259::PICS.lock().initialize();

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
        asm!("sti");
    }

    loop {
        halt!();
    }
}
