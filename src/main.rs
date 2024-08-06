mod list;
use std::env;

fn show_help() {
    println!("Usage: rvirsh [COMMAND]");
    println!("\nCommand:");
    println!("{:<10} {}", "help", "Show this help");
    println!("{:<10} {}", "list", "List all domains");
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
        _ => show_help(),
    };
}
