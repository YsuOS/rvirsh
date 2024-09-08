mod net_info;
mod net_list;
mod net_uuid;

use std::env;

use config::Config;
use virt::{connect::Connect, network::Network};

use crate::help::help_net;

pub fn main(settings: &Config, cmd: &str) {
    let uri = settings.get_string("URI").unwrap();
    let conn = Connect::open(Some(&uri)).unwrap();

    if cmd == "net-list" {
        net_list::list_net(&conn);
        return;
    }

    let net_name = env::args().nth(2);
    if net_name.is_none() {
        help_net(cmd);
        return;
    }

    let net_name = net_name.unwrap();
    let net = Network::lookup_by_name(&conn, &net_name).unwrap();

    match cmd {
        "net-uuid" => net_uuid::show_net_uuid(&net),
        "net-info" => net_info::show_net_info(&net),
        _ => eprintln!("{} is not supported", cmd),
    }
}

fn get_autostart_str(net: &Network) -> &str {
    if net.get_autostart().unwrap() {
        "yes"
    } else {
        "no"
    }
}

fn get_persistent_str(net: &Network) -> &str {
    if net.is_persistent().unwrap() {
        "yes"
    } else {
        "no"
    }
}
