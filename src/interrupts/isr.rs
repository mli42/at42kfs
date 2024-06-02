use crate::cli::{handle_cli_caret_blink, handle_cli_change, CliState, COMMAND_LINE_LENGTH};
use crate::interrupts::{pic8259, InterruptIndex, InterruptStackFrame};
use crate::keyboard::{handle_scancode, KeyboardState, KeymapLanguage};
use crate::println;

macro_rules! create_isr {
    ($handler_name:ident, $enum_value:expr) => {
        pub extern "x86-interrupt" fn $handler_name(frame: InterruptStackFrame) {
            println!("{:?} exception occured", $enum_value);
            println!("{:#x?}", frame);

            if ($enum_value != InterruptIndex::Breakpoint) {
                crate::halt!();
            }
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
create_isr!(math_fault_isr, InterruptIndex::MathFault);
create_isr!(alignment_check_isr, InterruptIndex::AlignmentCheck);
create_isr!(machine_check_isr, InterruptIndex::MachineCheck);
create_isr!(simdexception_isr, InterruptIndex::SIMDException);
create_isr!(
    virtualization_exception_isr,
    InterruptIndex::VirtualizationException
);
create_isr!(
    control_protection_exception_isr,
    InterruptIndex::ControlProtectionException
);
create_isr!(
    hypervisor_injection_exception_isr,
    InterruptIndex::HypervisorInjectionException
);
create_isr!(vmm_exception_isr, InterruptIndex::VMMException);
create_isr!(security_exception_isr, InterruptIndex::SecurityException);

pub static mut TIMER_TICKS: i8 = -1;

pub extern "x86-interrupt" fn timer_isr(_: InterruptStackFrame) {
    if unsafe { TIMER_TICKS } == -1 {
        handle_cli_change(unsafe { &mut CLI_STATE }, "");
    }

    if (unsafe { TIMER_TICKS } % 8 == 0) {
        unsafe { TIMER_TICKS = 0 };
        handle_cli_caret_blink(unsafe { &mut CLI_STATE });
    }

    unsafe { TIMER_TICKS += 1 };

    pic8259::PICS
        .lock()
        .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
}

pub static mut KEYBOARD_STATE: KeyboardState = KeyboardState {
    lang: KeymapLanguage::US,
    shift: false,
    ctrl: false,
    alt: false,
    capslock: false,
    verrnum: true,
};

pub static mut CLI_STATE: CliState = CliState {
    command_line: [b'\0'; COMMAND_LINE_LENGTH],
    caret_blink: false,
};

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_: InterruptStackFrame) {
    use crate::io::Port;

    let port = Port::new(0x60);
    let scancode: u8 = port.read();
    let mut scancode_changes = [b'\0'; 4];

    handle_scancode(
        scancode,
        unsafe { &mut KEYBOARD_STATE },
        &mut scancode_changes,
    );

    let clean_scancode_changes = crate::u8_to_str!(scancode_changes);
    handle_cli_change(unsafe { &mut CLI_STATE }, &clean_scancode_changes);

    pic8259::PICS
        .lock()
        .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
}
