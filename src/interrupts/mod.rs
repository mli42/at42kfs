mod idt;
mod isr;

use idt::{InterruptDescriptor, InterruptDescriptorTable, InterruptStackFrame};
use isr::*;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

        macro_rules! set_isr {
            ($handler_name:ident, $enum_value:expr) => {
                idt.set_descriptor(
                    $enum_value.as_usize(),
                    InterruptDescriptor::new($handler_name as u32, 0x8E),
                );
            };
        }

        set_isr!(division_by_zero_isr, InterruptIndex::DivisionByZero);
        set_isr!(debugger_isr, InterruptIndex::Debugger);
        set_isr!(nmi_isr, InterruptIndex::NMI);
        set_isr!(breakpoint_isr, InterruptIndex::Breakpoint);
        set_isr!(overflow_isr, InterruptIndex::Overflow);
        set_isr!(bounds_isr, InterruptIndex::Bounds);
        set_isr!(invalid_opcode_isr, InterruptIndex::InvalidOpcode);
        set_isr!(
            coprocessor_not_available_isr,
            InterruptIndex::CoprocessorNotAvailable
        );
        set_isr!(double_fault_isr, InterruptIndex::DoubleFault);
        set_isr!(
            coprocessor_segment_overrun_isr,
            InterruptIndex::CoprocessorSegmentOverrun
        );
        set_isr!(
            invalid_task_state_segment_isr,
            InterruptIndex::InvalidTaskStateSegment
        );
        set_isr!(segment_not_present_isr, InterruptIndex::SegmentNotPresent);
        set_isr!(stack_fault_isr, InterruptIndex::StackFault);
        set_isr!(
            general_protection_fault_isr,
            InterruptIndex::GeneralProtectionFault
        );
        set_isr!(page_fault_isr, InterruptIndex::PageFault);
        set_isr!(reserved_isr, InterruptIndex::Reserved);
        set_isr!(math_fault_isr, InterruptIndex::MathFault);
        set_isr!(alignment_check_isr, InterruptIndex::AlignmentCheck);
        set_isr!(machine_check_isr, InterruptIndex::MachineCheck);
        set_isr!(simdexception_isr, InterruptIndex::SIMDException);

        idt.ptr = idt::IDTR {
            base: &idt as *const _ as u32,
            limit: (core::mem::size_of::<InterruptDescriptor>() * 256 - 1) as u16,
        };

        idt
    };
}
