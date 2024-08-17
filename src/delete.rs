use config::Config;
use std::env;
use virt::connect::Connect;

fn show_help() {
    println!("Usage: rvirsh delete <domain>");
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

    crate::snapshot_delete::delete_all_snapshots(&conn, &dom_name);

    crate::undefine::undefine_domain(&conn, &dom_name);

    // TODO: Delete only a volume that matches the domain name
    let vol_name = dom_name.clone() + ".qcow2";
    let pool_name = settings.get_string("POOL").unwrap();

    crate::vol_delete::delete_volume(&conn, &pool_name, &vol_name);
}
