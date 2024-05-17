mod idt;

use crate::print;
use crate::println;
use idt::InterruptDescriptor;
use idt::InterruptDescriptorTable;
use lazy_static::lazy_static;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    DivisionByZero = 0x00,
    Debugger = 0x01,
    NMI = 0x02,
    Breakpoint = 0x03,
    Overflow = 0x04,
    Bounds = 0x05,
    InvalidOpcode = 0x06,
    CoprocessorNotAvailable = 0x07,
    DoubleFault = 0x08,
    CoprocessorSegmentOverrun = 0x09,
    InvalidTaskStateSegment = 0x0A,
    SegmentNotPresent = 0x0B,
    StackFault = 0x0C,
    GeneralProtectionFault = 0x0D,
    PageFault = 0x0E,
    Reserved = 0x0F,
    MathFault = 0x10,
    AlignmentCheck = 0x11,
    MachineCheck = 0x12,
    SIMDException = 0x13,
    // Timer = PIC_1_OFFSET,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub fn init_idt() {
    IDT.load();
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Initialise les descripteurs pour chaque type d'interruption
        // idt.set_descriptor(InterruptIndex::DivisionByZero.as_usize(), InterruptDescriptor32::new(0, 0x8E ));
        // idt.set_descriptor(InterruptIndex::Debugger.as_usize(), InterruptDescriptor32::new(0, 0x8E ));
        // idt.set_descriptor(InterruptIndex::NMI.as_usize(), InterruptDescriptor32::new(0, 0x8E ));
        // Et ainsi de suite...

        idt.set_descriptor(InterruptIndex::Breakpoint.as_usize(), InterruptDescriptor::new(breakpoint_handler as u32, 0x8F ));
        // idt.set_descriptor(InterruptIndex::Timer.as_usize(), InterruptDescriptor32::new(timer_interrupt_handler as u32, 0x8F ));
        idt.set_descriptor(InterruptIndex::DoubleFault.as_usize(), InterruptDescriptor::new(double_fault_handler as u32, 0x8F ));

        idt.ptr = idt::IDTR {
            base: &idt as *const _ as u32,
            limit: (core::mem::size_of::<InterruptDescriptorTable>() - 1) as u16,
        };

        idt
    };
}

extern "x86-interrupt" fn timer_interrupt_handler() {
    print!(".");
}

extern "x86-interrupt" fn double_fault_handler() {
    println!("Double fault handler");
}

extern "x86-interrupt" fn breakpoint_handler() {
    println!("Breakpoint interrupt occurred!");
}
