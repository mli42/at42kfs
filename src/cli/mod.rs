use crate::{println, WRITER};
use commands::{clear, divide_by_zero, echo, exit, help, hexdump, keymap, unknown_command};
use int::interrupt;

mod commands;
mod int;

pub const COMMAND_LINE_LENGTH: usize = crate::vga_buffer::BUFFER_WIDTH - PS1.len();
const ASCII_BACKSPACE: u8 = 0x08;
const ASCII_DELETE: u8 = 0x7f;
const PS1: &str = "> ";

type Handler = unsafe fn(_: &CliState) -> ();

const HANDLERS: &[(&str, Handler)] = &[
    ("help", help),
    ("echo", echo),
    ("clear", clear),
    ("hexdump", hexdump),
    ("keymap", keymap),
    ("exit", exit),
    ("divide_by_zero", divide_by_zero),
    ("int", interrupt),
];

pub struct CliState {
    pub command_line: [u8; COMMAND_LINE_LENGTH],
    pub caret_blink: bool,
}

fn get_handler(command_name: &str) -> Handler {
    for &(handler_name, handler_func) in HANDLERS {
        if handler_name == command_name {
            return handler_func;
        }
    }
    unknown_command
}

fn call_cli_handler(cli_state: &CliState) {
    let (_, mut argv) = crate::split_u8_string!(cli_state.command_line);

    if let Some(command_name) = argv.next() {
        unsafe {
            get_handler(command_name)(cli_state);
        }
    }
}

pub fn handle_cli_caret_blink(cli_state: &mut CliState) {
    let mut writer = WRITER.lock();
    let position = cli_state
        .command_line
        .iter()
        .position(|&c| c == b'\0')
        .unwrap_or(0)
        + PS1.len();

    writer.column_position = position;

    let new_foreground = writer.color_code.get_background();
    let new_background = writer.color_code.get_foreground();

    if cli_state.caret_blink {
        writer.set_colors(Some(new_foreground), Some(new_background));
    }

    writer.write_byte(b' ');

    if cli_state.caret_blink {
        writer.set_colors(Some(new_background), Some(new_foreground));
    }

    cli_state.caret_blink = !cli_state.caret_blink;
}

fn write_command_line(cli_state: &CliState) {
    let mut writer = WRITER.lock();
    let command_line_index = crate::get_array_end_index!(cli_state.command_line);

    writer.column_position = 0;
    writer.write_string(PS1);
    writer.write_string(&crate::u8_to_str!(cli_state.command_line));
    for _ in 0..COMMAND_LINE_LENGTH - command_line_index {
        writer.write_byte(b' ');
    }
}

pub fn handle_cli_change(cli_state: &mut CliState, change_str: &str) {
    if change_str == "\n" {
        println!();

        call_cli_handler(cli_state);
        cli_state.command_line = [b'\0'; COMMAND_LINE_LENGTH];
        return;
    }

    let mut command_line_index = crate::get_array_end_index!(cli_state.command_line);

    for c in change_str.bytes() {
        if c == ASCII_BACKSPACE || c == ASCII_DELETE {
            if command_line_index > 0 {
                command_line_index -= 1;
                cli_state.command_line[command_line_index] = b'\0';
            }
            continue;
        }

        if command_line_index >= COMMAND_LINE_LENGTH - 1 {
            continue;
        }

        cli_state.command_line[command_line_index] = c;
        command_line_index += 1;
    }
    write_command_line(cli_state);
}
