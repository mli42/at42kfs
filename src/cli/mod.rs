use crate::println;

pub fn clear() {
    for _ in 0..25 {
        println!();
    }
}

pub fn echo(args: &[u8]) {
    if args.len() == 0 {
        println!("echo: missing string argument");
    } else {
        println!("{}", args as &str);
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

pub fn handle_cli_change(state: &mut CliState, change_str: &[u8]) {
    if change_str[0] == b'\n' {
        println!();

        // let shit = str::from_utf8(&state.command_line);

        let help_as_bytes = "help\0".as_bytes();
        let echo_as_bytes = "echo ".as_bytes();
        let clear_as_bytes = "clear\0".as_bytes();

        match state.command_line {
            help_as_bytes => help(),
            echo_as_bytes => echo(&state.command_line[5..]),
            clear_as_bytes => clear(),
            _ => unknown_command(state.command_line),
        }
    } else {
        str.push_str(change_str);
    }
}
