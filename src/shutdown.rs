use config::Config;
use std::env;
use virt::{connect::Connect, domain::Domain};

fn show_help() {
    println!("Usage: rvirsh shutdown <domain>");
}

fn shutdown_domain(dom: &Domain) {
    dom.shutdown().unwrap();
    println!("Domain {} shutdowned", &dom.get_name().unwrap());
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

    let dom = Domain::lookup_by_name(&conn, &dom_name).unwrap();
    shutdown_domain(&dom);
}
