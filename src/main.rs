mod list;
mod poweroff;
mod start;

use config::Config;
use std::env;

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

    let settings = Config::builder()
        .add_source(config::File::with_name("default"))
        .build()
        .unwrap();

    if command.is_none() {
        println!("1st argument is required");
        show_help();
    }

    let command = command.unwrap();
    match command.as_str() {
        "list" => list::main(&settings),
        "start" => start::main(&settings),
        "poweroff" => poweroff::main(&settings),
        _ => show_help(),
    };
}
