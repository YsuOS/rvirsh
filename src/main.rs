mod delete;
mod list;
mod poweroff;
mod snapshot_delete;
mod start;
mod undefine;
mod vol_delete;
mod vol_list;

use config::Config;
use std::env;

fn show_help() {
    let width = 20;
    println!("Usage: rvirsh [COMMAND]");
    println!("\nCommands:");
    println!("{:<width$} {}", "help", "Show this help");
    println!("{:<width$} {}", "list", "List all domains");
    println!("{:<width$} {}", "start", "Start domain");
    println!("{:<width$} {}", "poweroff", "Forcefully terminate domain");
    println!(
        "{:<width$} {}",
        "delete", "Run 'undefine', 'vol-delete', and 'snapshot-delete'"
    );
    println!("{:<width$} {}", "undefine", "undefine domain");
    println!("{:<width$} {}", "vol-list", "List all volumes");
    println!("{:<width$} {}", "vol-delete", "Delete volume");
    println!(
        "{:<width$} {}",
        "snapshot-delete", "Delete a domain snapshots"
    );

    println!("\nDepricated Command:");
    println!("{:<width$} {}", "destroy", "Use 'poweroff'");
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
        "delete" => delete::main(&settings),
        "undefine" => undefine::main(&settings),
        "vol-delete" => vol_delete::main(&settings),
        "vol-list" => vol_list::main(&settings),
        "snapshot-delete" => snapshot_delete::main(&settings),
        _ => show_help(),
    };
}
