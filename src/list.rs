use config::Config;
use virt::{
    connect::Connect,
    domain::Domain,
    sys::{VIR_DOMAIN_PAUSED, VIR_DOMAIN_RUNNING, VIR_DOMAIN_SHUTOFF},
};

fn get_state_str(dom: &Domain) -> &str {
    match dom.get_state().unwrap() {
        (VIR_DOMAIN_RUNNING, _) => "running",
        (VIR_DOMAIN_PAUSED, _) => "paused",
        (VIR_DOMAIN_SHUTOFF, _) => "shut off",
        // TODO: support other states on demand
        _ => "-",
    }
}

pub fn main(settings: &Config) {
    let uri = settings.get_string("URI").unwrap();

    let conn = Connect::open(Some(&uri)).unwrap();
    let doms = conn.list_all_domains(0).unwrap();

    println!("{:<20} {}", "Instance Name", "State");
    println!("{:<20} {}", "--------------------", "--------");
    for dom in doms {
        let name = dom.get_name().unwrap();
        let state = get_state_str(&dom);

        println!("{:<20} {}", name, state);
    }
}
