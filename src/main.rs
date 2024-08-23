mod delete;
mod list;
mod pool_list;
mod poweroff;
mod reboot;
mod resume;
mod shutdown;
mod snapshot_delete;
mod start;
mod suspend;
mod undefine;
mod vol_delete;
mod vol_list;

use config::Config;
use std::env;

fn show_help() {
    let width = 20;
    let helps: Vec<[&str; 2]> = vec![
        ["help", "Show this help"],
        ["list", "List all domains"],
        ["start", "Start domain"],
        ["shutdown", "Shutdown domain"],
        ["reboot", "Reboot domain"],
        ["suspend", "Suspend domain"],
        ["resume", "Resume domain"],
        ["poweroff", "Forcefully terminate domain"],
        [
            "delete",
            "Run 'undefine', 'vol-delete', and 'snapshot-delete'",
        ],
        ["undefine", "undefine domain"],
        ["vol-list", "List all volumes"],
        ["vol-delete", "Delete volume"],
        ["snapshot-delete", "Delete a domain snapshots"],
        ["pool-list", "List all pools"],
    ];

    println!("Usage: rvirsh [COMMAND]");
    println!("\nCommands:");
    for h in helps {
        println!("{:<width$} {}", h[0], h[1]);
    }

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
        "shutdown" => shutdown::main(&settings),
        "reboot" => reboot::main(&settings),
        "suspend" => suspend::main(&settings),
        "resume" => resume::main(&settings),
        "poweroff" => poweroff::main(&settings),
        "delete" => delete::main(&settings),
        "undefine" => undefine::main(&settings),
        "vol-delete" => vol_delete::main(&settings),
        "vol-list" => vol_list::main(&settings),
        "snapshot-delete" => snapshot_delete::main(&settings),
        "pool-list" => pool_list::main(&settings),
        "destroy" => println!("'destroy' is deprecated. use 'poweroff'"),
        _ => show_help(),
    };
}
