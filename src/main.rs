mod delete;
mod domain;
mod help;
mod hostinfo;
mod hostname;
mod net;
mod pool;
mod snapshot;
mod uri;
mod version;
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
        "list" | "start" | "shutdown" | "reboot" | "suspend" | "resume" | "reset" | "poweroff"
        | "undefine" | "dominfo" | "info" | "domid" | "domuuid" | "autostart" | "noautostart"
        | "domstate" | "dumpxml" => domain::main(&settings, command),
        "delete" => delete::main(&settings),
        "net-list" | "net-uuid" | "net-info" | "net-dumpxml" | "net-autostart"
        | "net-noautostart" | "net-stop" => net::main(&settings, command),
        "net-destroy" => println!("'net-destroy' is deprecated. use 'net-stop'"),
        "vol-delete" | "vol-list" | "vol-info" | "vol-path" | "vol-key" | "vol-dumpxml"
        | "vol-pool" | "vol-wipe" => volume::main(&settings, command),
        "snapshot-list" | "snapshot-delete" | "snapshot-info" | "snapshot-parent"
        | "snapshot-dumpxml" => snapshot::main(&settings, command),
        "pool-list" | "pool-info" | "pool-refresh" | "pool-uuid" | "pool-stop" | "pool-delete"
        | "pool-undefine" | "pool-clean" | "pool-autostart" | "pool-noautostart"
        | "pool-dumpxml" => pool::main(&settings, command),
        "pool-destroy" => println!("'pool-destroy' is deprecated. use 'pool-stop'"),
        "destroy" => println!("'destroy' is deprecated. use 'poweroff'"),
        "version" => version::main(&settings),
        "hostname" => hostname::main(&settings),
        "nodeinfo" => println!("'nodeinfo' is deprecated. use 'hostinfo'"),
        "hostinfo" => hostinfo::main(&settings),
        "uri" => uri::main(&settings),
        _ => help::show_help(),
    };
}
