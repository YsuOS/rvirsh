use config::Config;
use std::env;
use virt::{connect::Connect, domain::Domain};

fn show_help() {
    println!("Usage: rvirsh undefine <domain>");
}

pub fn undefine_domain(dom: &Domain) {
    if dom.is_active().unwrap() {
        dom.destroy().unwrap();
    }

    dom.undefine().unwrap();
    println!("Domain {} is undefined", dom.get_name().unwrap());
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
    let dom = Domain::lookup_by_name(&conn, &dom_name).unwrap();

    undefine_domain(&dom);
}
