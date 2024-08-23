use config::Config;
use std::env;
use virt::{connect::Connect, domain::Domain};

fn show_help() {
    println!("Usage: rvirsh reboot <domain>");
}

fn reboot_domain(dom: &Domain) {
    if !dom.is_active().unwrap() {
        println!("Domain {} is inactive", &dom.get_name().unwrap());
        return;
    }

    dom.reboot(0).unwrap();
    println!("Domain {} rebooted", &dom.get_name().unwrap());
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
    reboot_domain(&dom);
}
