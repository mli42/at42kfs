use crate::cli::CliState;
use crate::{println, WRITER};

pub fn unknown_command(cli_state: &CliState) {
    let (_, mut argv) = crate::split_u8_string!(cli_state.command_line);

    println!("Unknown command: \"{}\"", argv.next().unwrap_or_default());
    println!("Type 'help' for a list of available commands");
}

pub fn help(_: &CliState) {
    println!("Available commands:");
    println!("- help: Display this help message");
    println!("- echo <string>: Echo the string back to the console");
    println!("- hexdump <addr?> <size?>: Hexdump the memory at the given address for a given number of bytes");
    println!("- clear: Clear the console");
}

pub fn clear(_: &CliState) {
    let mut writer = WRITER.lock();

    for i in 0..25 {
        writer.clear_row(i);
    }
}

pub fn echo(cli_state: &CliState) {
    let argv = crate::u8_to_str!(cli_state.command_line);

    println!("{}", argv[4..].trim());
}

pub fn hexdump(cli_state: &CliState) {
    let (argc, mut argv) = crate::split_u8_string!(cli_state.command_line);

    let c = 42;
    let addr_str = argv.nth(1).unwrap_or_default();
    let without_prefix = addr_str.trim_start_matches("0x");

    let addr =
        u32::from_str_radix(without_prefix, 16).unwrap_or(&c as *const i32 as u32) as *const u8;

    let size = argv.next().unwrap_or("80").parse::<u32>().unwrap_or(80) as usize;

    crate::hexdump(addr, size);
}
