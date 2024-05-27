use crate::{println, WRITER};
use commands::{clear, echo, help, hexdump, unknown_command};

mod commands;

const COMMAND_LINE_LENGTH: usize = 80;
const ASCII_BACKSPACE: u8 = 0x08;
const ASCII_DELETE : u8 = 0x7f;

type Handler = fn(_: &CliState) -> ();

const HANDLERS: &[(&str, Handler)] = &[
    ("help", help),
    ("echo", echo),
    ("clear", clear),
    ("hexdump", hexdump),
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
        get_handler(command_name)(cli_state);
    }
}

pub fn handle_cli_caret_blink(cli_state: &mut CliState) {
    let mut writer = WRITER.lock();
    let position = (cli_state
        .command_line
        .iter()
        .position(|&c| c == b'\0')
        .unwrap_or(0) as i32
        - 1)
    .max(0) as usize;

    writer.column_position = (position + 2).max(2);

    let new_foreground = writer.color_code.get_background();
    let new_background = writer.color_code.get_foreground();

    if cli_state.caret_blink {
        writer.set_colors(Some(new_foreground), Some(new_background));
    }

    writer.write_byte(cli_state.command_line[position]);

    if cli_state.caret_blink {
        writer.set_colors(Some(new_background), Some(new_foreground));
    }

    cli_state.caret_blink = !cli_state.caret_blink;
}

fn write_command_line(cli_state: &CliState) {
    let mut writer = WRITER.lock();
    let command_line_index = crate::get_array_end_index!(cli_state.command_line);

    writer.column_position = 0;
    writer.write_string("> ");
    writer.write_string(&crate::u8_to_str!(cli_state.command_line));
    for _ in 0..COMMAND_LINE_LENGTH - command_line_index - 2 {
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

        if command_line_index >= COMMAND_LINE_LENGTH {
            continue;
        }

        cli_state.command_line[command_line_index] = c;
        command_line_index += 1;
    }
    write_command_line(cli_state);
}
