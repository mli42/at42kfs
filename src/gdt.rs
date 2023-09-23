use core::arch::asm;
use core::ptr::copy_nonoverlapping;

use crate::print;
use crate::println;
use crate::vga_buffer::hexdump;

#[repr(C, packed)]
struct SegmentDescriptor {
    limit_low: u16,  // Limite 0-15
    base_low: u16,   // Base 0-15
    base_middle: u8, // Base 16-23
    access: u8,      // Type, S, DPL, P
    granularity: u8, // L, DB, G, Limit 16-19
    base_high: u8,   // Base 24-31
}

impl SegmentDescriptor {
    fn new(base: u32, limit: u32, access: u8, granularity: u8) -> Self {
        SegmentDescriptor {
            base_low: base as u16,
            base_middle: (base >> 16) as u8,
            base_high: (base >> 24) as u8,
            limit_low: limit as u16,
            granularity: ((limit >> 16) as u8 & 0x0F) | (granularity & 0xF0),
            access: access,
        }
    }
}

#[repr(C, packed)]
pub struct GlobalDescriptorTable {
    null_segment: SegmentDescriptor,
    code_segment: SegmentDescriptor,
    data_segment: SegmentDescriptor,
    stack_segment: SegmentDescriptor, // Descripteur de segment null, toujours 0
    user_code_segment: SegmentDescriptor,
    user_data_segment: SegmentDescriptor,
    user_stack_segment: SegmentDescriptor,
}

impl GlobalDescriptorTable {
    pub fn init() -> Self {
        GlobalDescriptorTable {
            null_segment: SegmentDescriptor::new(0, 0, 0, 0),
            code_segment: SegmentDescriptor::new(0, 0xFFFFF, 0b10011010, 0b11001111),
            data_segment: SegmentDescriptor::new(0, 0xFFFFF, 0b10010010, 0b11001111),
            stack_segment: SegmentDescriptor::new(0, 0xFFFFF, 0b10010111, 0b11001111),
            user_code_segment: SegmentDescriptor::new(0, 0xFFFFF, 0xFA, 0b11001111),
            user_data_segment: SegmentDescriptor::new(0, 0xFFFFF, 0xF2, 0b11001111),
            user_stack_segment: SegmentDescriptor::new(0, 0xFFFFF, 0xF2, 0b11001111),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct GdtPointer {
    limit: u16,
    base: u32,
}

impl GlobalDescriptorTable {
    pub fn install(&self) {
        let gdt_pointer = GdtPointer {
            limit: (core::mem::size_of::<GlobalDescriptorTable>() - 1) as u16,
            base: 0x800,
        };

        unsafe {
            copy_nonoverlapping(
                self as *const _ as *mut u8,
                0x800 as *mut u8,
                gdt_pointer.limit as usize + 1,
            );
        }

        unsafe {
            asm!("lgdt [{x}]", x = in(reg) &gdt_pointer,);
        }
    }

    pub fn get_gdt_pointer(&self) -> GdtPointer {
        let gdt_pointer = GdtPointer { limit: 0, base: 0 };

        unsafe {
            asm!("sgdt [{x}]", x = in(reg) & gdt_pointer,);
        }

        return gdt_pointer;
    }

    pub fn print(&self) {
        let gdt_pointer = self.get_gdt_pointer();

        hexdump(gdt_pointer.base as *const u8, gdt_pointer.limit as usize);
    }
}
