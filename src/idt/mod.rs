mod idt;
pub mod pic8259;

use crate::println;
use idt::InterruptDescriptor32;
use idt::InterruptDescriptorTable;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

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

    Timer = PIC_1_OFFSET,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

use crate::print;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Initialise les descripteurs pour chaque type d'interruption
        idt.set_descriptor(InterruptIndex::DivisionByZero.as_usize(), InterruptDescriptor32::new(0, 0x8E ));
        idt.set_descriptor(InterruptIndex::Debugger.as_usize(), InterruptDescriptor32::new(0, 0x8E ));
        idt.set_descriptor(InterruptIndex::NMI.as_usize(), InterruptDescriptor32::new(0, 0x8E ));
        // Et ainsi de suite...

        // Initialise le descripteur pour l'interruption "Breakpoint" avec un callback approprié
        idt.set_descriptor(InterruptIndex::Breakpoint.as_usize(), InterruptDescriptor32::new(breakpoint_handler as u32, 0x8F ));
        idt.set_descriptor(InterruptIndex::Timer.as_usize(), InterruptDescriptor32::new(timer_interrupt_handler as u32, 0x8F ));
        idt.set_descriptor(InterruptIndex::DoubleFault.as_usize(), InterruptDescriptor32::new(timer_interrupt_handler as u32, 0x8F ));

        use core::mem::size_of;
        idt.ptr = idt::IDTR {
            base: &idt as *const _ as u32,
            limit: (size_of::<InterruptDescriptorTable>() - 1) as u16,
        };

        idt
    };
}

extern "x86-interrupt" fn timer_interrupt_handler() {
    print!(".");
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

// Fonction de callback pour l'interruption "Breakpoint"
extern "x86-interrupt" fn breakpoint_handler() {
    // Votre logique de gestion de l'interruption "Breakpoint" ici
    // Par exemple, affichage d'un message de débogage
    println!("Breakpoint interrupt occurred!");
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Breakpoint.as_u8());
    }
}

pub fn init_idt() {
    IDT.load();
}
