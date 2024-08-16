mod list;
mod poweroff;
mod start;

use std::env;

pub const URI: &str = "qemu:///system";

fn show_help() {
    println!("Usage: rvirsh [COMMAND]");
    println!("\nCommands:");
    println!("{:<10} {}", "help", "Show this help");
    println!("{:<10} {}", "list", "List all domains");
    println!("{:<10} {}", "start", "Start domain");
    println!("{:<10} {}", "poweroff", "Forcefully terminate domain");

    println!("\nDepricated Command:");
    println!("{:<10} {}", "destroy", "Use 'poweroff'");
}

fn main() {
    let command: Option<String> = env::args().nth(1);

    if command.is_none() {
        println!("1st argument is required");
        show_help();
    }

    let command = command.unwrap();
    match command.as_str() {
        "list" => list::main(),
        "start" => start::main(),
        "poweroff" => poweroff::main(),
        _ => show_help(),
    };
}
