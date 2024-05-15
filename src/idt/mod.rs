
mod pic8259;
mod idt;

// in src/interrupts.rs

use pic8259::ChainedPics;
use spin;
use lazy_static::lazy_static;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

    // in src/interrupts.rs

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
    static ref IDT: idt::InterruptDescriptorTable = {
        let mut idt = idt::InterruptDescriptorTable::new();
        idt.set_descriptor(0x03, InterruptDescriptor32::new())
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler); // new

        idt
    };
}

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    print!(".");
}

// Fonction de callback pour l'interruption "Breakpoint"
extern "x86-interrupt" fn breakpoint_handler() {
    // Votre logique de gestion de l'interruption "Breakpoint" ici
    // Par exemple, affichage d'un message de débogage
    println!("Breakpoint interrupt occurred!");
}


pub fn init_idt() {
    let mut idt = InterruptDescriptorTable::new();

    // Initialise les descripteurs pour chaque type d'interruption
    idt.set_descriptor(InterruptIndex::DivisionByZero, 0, 0, 0x8E);
    idt.set_descriptor(InterruptIndex::Debugger, 0, 0, 0x8E);
    idt.set_descriptor(InterruptIndex::NMI, 0, 0, 0x8E);
    // Et ainsi de suite...

    // Initialise le descripteur pour l'interruption "Breakpoint" avec un callback approprié
    let breakpoint_offset = breakpoint_handler as u32;
    idt.set_descriptor(InterruptIndex::Breakpoint, breakpoint_offset, 0, 0x8F);
}