use crate::println;

pub fn clear() {
    for _ in 0..25 {
        println!();
    }
}

pub fn echo(args: &str) {
    if args.len() == 0 {
        println!("echo: missing string argument");
    } else {
        println!("{}", args);
    }
}

pub fn unknown_command(command: &str) {
    println!("Unknown command: {}", command);
    println!("Type 'help' for a list of available commands");
}

pub fn help() {
    println!("Available commands:");
    println!("help: Display this help message");
    println!("echo <string>: Echo the string back to the console");
    println!("clear: Clear the console");
}

pub struct CliState {
    pub command_line: [u8; 80],
}

pub fn handle_cli_change(cli_state: &mut CliState, change_str: &str) {
    let command_line = crate::u8_to_str!(cli_state.command_line);
    let mut command_line_index = crate::get_array_end_index!(cli_state.command_line);

    if change_str == "\n" {
        println!();

        match command_line {
            "help" => help(),
            s if s.starts_with("echo") => echo(&command_line[4..]),
            "clear" => clear(),
            _ => unknown_command(command_line),
        }
        cli_state.command_line = [b'\0'; 80];
    } else {
        for c in change_str.bytes() {
            if command_line_index >= 80 {
                break;
            }
            cli_state.command_line[command_line_index] = c;
            command_line_index += 1;
        }
    }
}
