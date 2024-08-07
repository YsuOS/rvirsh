use crate::URI;
use std::env;
use virt::{connect::Connect, domain::Domain};

fn show_help() {
    println!("Usage: rvirsh start <domain>");
}

fn start_domain(conn: &Connect, name: &str) {
    let dom = Domain::lookup_by_name(conn, name).unwrap();
    dom.create().unwrap();
}

pub fn main() {
    let dom_name = env::args().nth(2);

    if dom_name.is_none() {
        eprintln!("Domain name is required");
        show_help();
        return;
    }

    let dom_name = dom_name.unwrap();
    let conn = Connect::open(Some(URI)).unwrap();

    start_domain(&conn, &dom_name);
    println!("Domain {} started", &dom_name);
}
