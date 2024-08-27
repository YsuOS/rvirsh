use config::Config;
use std::env;
use virt::{connect::Connect, domain::Domain};

pub mod snapshot_delete;
mod snapshot_list;

use crate::help::help_domain;

pub fn main(settings: &Config, cmd: &str) {
    let uri = settings.get_string("URI").unwrap();
    let conn = Connect::open(Some(&uri)).unwrap();

    let dom_name = env::args().nth(2);
    if dom_name.is_none() {
        help_domain(cmd);
        return;
    }

    let dom_name = dom_name.unwrap();
    let dom = Domain::lookup_by_name(&conn, &dom_name).unwrap();

    match cmd {
        "snapshot-delete" => snapshot_delete::delete_all_snapshots(&dom),
        "snapshot-list" => snapshot_list::list_snapshots(&dom),
        _ => eprintln!("{} is not supported", cmd),
    }
}
