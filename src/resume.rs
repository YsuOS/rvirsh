use config::Config;
use std::env;
use virt::{connect::Connect, domain::Domain, sys::VIR_DOMAIN_PAUSED};

fn show_help() {
    println!("Usage: rvirsh resume <domain>");
}

fn resume_domain(dom: &Domain) {
    let (state, _) = dom.get_state().unwrap();
    if state != VIR_DOMAIN_PAUSED {
        println!("Domain {} is not paused", &dom.get_name().unwrap());
        return;
    }

    dom.resume().unwrap();
    println!("Domain {} resumed", &dom.get_name().unwrap());
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
    resume_domain(&dom);
}
