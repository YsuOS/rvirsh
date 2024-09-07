use config::Config;
use std::env;
use virt::{connect::Connect, domain::Domain, domain_snapshot::DomainSnapshot};

pub mod snapshot_delete;
mod snapshot_info;
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
        "snapshot-info" => {
            let snapshot_name = env::args().nth(3);
            if snapshot_name.is_none() {
                eprintln!("snapshot name is required");
                eprintln!("Usage: rvirsh {} <domain> <snapshot>", cmd);
                return;
            }
            let snapshot_name = snapshot_name.unwrap();
            let snapshot = DomainSnapshot::lookup_by_name(&dom, &snapshot_name, 0).unwrap();

            snapshot_info::show_snapshot_info(&dom, &snapshot)
        }
        _ => eprintln!("{} is not supported", cmd),
    }
}
