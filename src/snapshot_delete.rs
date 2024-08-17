use config::Config;
use std::env;
use virt::{connect::Connect, domain::Domain};

fn show_help() {
    println!("Usage: rvirsh snapshot-delete <domain>");
}

fn delete_all_snapshots(conn: &Connect, dom_name: &str) {
    let dom = Domain::lookup_by_name(conn, dom_name).unwrap();
    let snapshots = dom.list_all_snapshots(0).unwrap();

    for snapshot in snapshots {
        snapshot.delete(0).unwrap();
    }
}

pub fn main(settings: &Config) {
    let dom_name = env::args().nth(2);

    if dom_name.is_none() {
        eprintln!("Domain name is required");
        show_help();
        return;
    }

    let uri = settings.get_string("URI").unwrap();
    let conn = Connect::open(Some(&uri)).unwrap();

    let dom_name = dom_name.unwrap();

    delete_all_snapshots(&conn, &dom_name);

    println!("All snapshots of {} is deleted", &dom_name);
}
