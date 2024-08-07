use config::Config;
use std::env;
use virt::{connect::Connect, domain::Domain};

fn show_help() {
    println!("Usage: rvirsh poweroff <domain>");
}

fn poweroff_domain(conn: &Connect, name: &str) {
    let dom = Domain::lookup_by_name(conn, name).unwrap();

    if !dom.is_active().unwrap() {
        println!("Domain {} is inactive", name);
        return;
    }

    dom.destroy().unwrap();
    println!("Domain {} is powered off", name);
}

pub fn main(settings: &Config) {
    let dom_name = env::args().nth(2);

    if dom_name.is_none() {
        eprintln!("Domain name is required");
        show_help();
        return;
    }

    let uri = settings.get_string("URI").unwrap();

    let dom_name = dom_name.unwrap();
    let conn = Connect::open(Some(&uri)).unwrap();

    poweroff_domain(&conn, &dom_name);
}
