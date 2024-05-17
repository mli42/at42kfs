use crate::interrupts::{InterruptIndex, InterruptStackFrame};
use crate::println;

macro_rules! create_isr {
    ($handler_name:ident, $enum_value:expr) => {
        pub extern "x86-interrupt" fn $handler_name(frame: InterruptStackFrame) {
            println!("{:?} exception occured", $enum_value);

            if ($enum_value == InterruptIndex::Breakpoint) {
                println!("AYAAA");
            }

            println!("{:#x?}", frame);
        }
    };
}

create_isr!(division_by_zero_isr, InterruptIndex::DivisionByZero);
create_isr!(debugger_isr, InterruptIndex::Debugger);
create_isr!(nmi_isr, InterruptIndex::NMI);
create_isr!(breakpoint_isr, InterruptIndex::Breakpoint);
create_isr!(overflow_isr, InterruptIndex::Overflow);
create_isr!(bounds_isr, InterruptIndex::Bounds);
create_isr!(invalid_opcode_isr, InterruptIndex::InvalidOpcode);
create_isr!(
    coprocessor_not_available_isr,
    InterruptIndex::CoprocessorNotAvailable
);
create_isr!(double_fault_isr, InterruptIndex::DoubleFault);
create_isr!(
    coprocessor_segment_overrun_isr,
    InterruptIndex::CoprocessorSegmentOverrun
);
create_isr!(
    invalid_task_state_segment_isr,
    InterruptIndex::InvalidTaskStateSegment
);
create_isr!(segment_not_present_isr, InterruptIndex::SegmentNotPresent);
create_isr!(stack_fault_isr, InterruptIndex::StackFault);
create_isr!(
    general_protection_fault_isr,
    InterruptIndex::GeneralProtectionFault
);
create_isr!(page_fault_isr, InterruptIndex::PageFault);
create_isr!(reserved_isr, InterruptIndex::Reserved);
create_isr!(math_fault_isr, InterruptIndex::MathFault);
create_isr!(alignment_check_isr, InterruptIndex::AlignmentCheck);
create_isr!(machine_check_isr, InterruptIndex::MachineCheck);
create_isr!(simdexception_isr, InterruptIndex::SIMDException);
