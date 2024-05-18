//! Provides types for the Interrupt Descriptor Table and its entries.

/// Represents the interrupt stack frame pushed by the CPU on interrupt or exception entry.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u32,
    pub code_segment: u32,
    pub cpu_flags: u32,
    pub stack_pointer: u32,
    pub stack_segment: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct InterruptDescriptor {
    isr_low: u16,        // The lower 16 bits of the ISR's address
    kernel_cs: u16, // The GDT segment selector that the CPU will load into CS before calling the ISR
    zero: u8,       // Reserved
    type_attributes: u8, // Type and attributes; see the IDT doc
    isr_high: u16,  // The higher 16 bits of the ISR's address
}

impl InterruptDescriptor {
    pub fn new(offset: u32, type_attributes: u8) -> Self {
        Self {
            isr_low: (offset & 0xFFFF) as u16,
            kernel_cs: 0x8,
            zero: 0,
            type_attributes,
            isr_high: ((offset >> 16) & 0xFFFF) as u16,
        }
    }
}

// Structure représentant la table des descripteurs d'interruption
#[repr(C, align(0x10))]
pub struct InterruptDescriptorTable {
    pub descriptors: [InterruptDescriptor; 256],
    pub ptr: IDTR,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IDTR {
    pub limit: u16,
    pub base: u32,
}

impl InterruptDescriptorTable {
    pub fn new() -> Self {
        Self {
            descriptors: [InterruptDescriptor::new(0, 0); 256],
            ptr: IDTR { limit: 0, base: 0 },
        }
    }

    pub fn set_descriptor(&mut self, index: usize, descriptor: InterruptDescriptor) {
        self.descriptors[index] = descriptor;
    }

    pub fn load(&'static self) {
        unsafe {
            core::arch::asm!(
                "lidt [{}]",
                in(reg) &self.ptr as *const _ as u32
            );
        }
    }
}
