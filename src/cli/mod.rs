use crate::{println, WRITER};

pub fn clear() {
    let mut writer = WRITER.lock();

    for i in 0..25 {
        writer.clear_row(i);
    }
}

pub fn echo(args: &str) {
    if args.len() == 0 {
        println!("echo: missing string argument");
    } else {
        println!("{}", args.trim_start());
    }
}

pub fn unknown_command(command: &str) {
    println!("Unknown command: \"{}\"", command);
    println!("Type 'help' for a list of available commands");
}

pub fn help() {
    println!("Available commands:");
    println!("help: Display this help message");
    println!("echo <string>: Echo the string back to the console");
    println!("hexdump <addr?> <size?>: Hexdump the memory at the given address for a given number of bytes");
    println!("clear: Clear the console");
}

pub fn hexdump(args: &str) {
    let c = 42;
    let mut args_splitted = args.trim_start().split(' ');
    let addr_str = args_splitted.next().unwrap_or_default();

    let without_prefix = addr_str.trim_start_matches("0x");

    let addr =
        u32::from_str_radix(without_prefix, 16).unwrap_or(&c as *const i32 as u32) as *const u8;

    let size = args_splitted
        .next()
        .unwrap_or("80")
        .parse::<u32>()
        .unwrap_or(80) as usize;

    crate::hexdump(addr, size);
}

pub struct CliState {
    pub command_line: [u8; 80],
    pub caret_blink: bool,
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

pub fn handle_cli_change(cli_state: &mut CliState, change_str: &str) {
    let command_line = crate::u8_to_str!(cli_state.command_line);
    let mut command_line_index = crate::get_array_end_index!(cli_state.command_line);

    if change_str == "\n" {
        println!();

        let command_name = command_line.split_whitespace().next().unwrap_or("");

        if command_name.len() == 0 {
            return;
        }

        match command_line {
            "help" => help(),
            s if s.starts_with("echo") => echo(&command_line[4..]),
            "clear" => clear(),
            s if s.starts_with("hexdump") => hexdump(&command_line[7..]),
            _ => unknown_command(command_line),
        }
        cli_state.command_line = [b'\0'; 80];
    } else {
        for c in change_str.bytes() {
            if c == 0x08 || c == 0x7f {
                if command_line_index > 0 {
                    command_line_index -= 1;
                    cli_state.command_line[command_line_index] = b'\0';
                }
                continue;
            }

            if command_line_index >= 80 {
                continue;
            }

            cli_state.command_line[command_line_index] = c;
            command_line_index += 1;
        }

        let mut writer = WRITER.lock();
        writer.column_position = 0;
        writer.write_string("> ");
        writer.write_string(&crate::u8_to_str!(cli_state.command_line));
        for _ in 0..80 - command_line_index - 2 {
            writer.write_byte(b' ');
        }
    }
}
