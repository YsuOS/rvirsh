mod delete;
mod domain;
mod help;
mod pool_list;
mod snapshot_delete;
mod volume;

use config::Config;
use std::env;

fn main() {
    let command: Option<String> = env::args().nth(1);

    let settings = Config::builder()
        .add_source(config::File::with_name("default"))
        .build()
        .unwrap();

    if command.is_none() {
        println!("1st argument is required");
        help::show_help();
    }

    let command = command.unwrap();
    let command = command.as_str();
    match command {
        "list" | "start" | "shutdown" | "reboot" | "suspend" | "resume" | "poweroff"
        | "undefine" => domain::main(&settings, command),
        "delete" => delete::main(&settings),
        "vol-delete" | "vol-list" => volume::main(&settings, command),
        "snapshot-delete" => snapshot_delete::main(&settings),
        "pool-list" => pool_list::main(&settings),
        "destroy" => println!("'destroy' is deprecated. use 'poweroff'"),
        _ => help::show_help(),
    };
}
