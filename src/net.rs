mod net_autostart;
mod net_clean;
mod net_create;
mod net_define;
mod net_dumpxml;
mod net_info;
mod net_list;
mod net_noautostart;
mod net_start;
mod net_stop;
mod net_undefine;
mod net_uuid;

use std::env;

use anyhow::Result;
use config::Config;
use virt::network::Network;

use crate::{get_conn, get_xml, help::help_net};

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    if cmd == "net-list" {
        net_list::list_net(&conn);
        return Ok(());
    } else if cmd == "net-define" || cmd == "net-create" {
        let mut xml = get_xml(cmd)?;

        if cmd == "net-define" {
            net_define::define_net(&conn, &mut xml);
        } else if cmd == "net-create" {
            net_create::create_net(&conn, &mut xml);
        }
        return Ok(());
    }

    let net_name = env::args().nth(2);
    if net_name.is_none() {
        help_net(cmd);
        return Ok(());
    }

    let net_name = net_name.unwrap();
    let net = Network::lookup_by_name(&conn, &net_name).unwrap();

    match cmd {
        "net-autostart" => net_autostart::autostart_net(&net),
        "net-noautostart" => net_noautostart::noautostart_net(&net),
        "net-stop" => net_stop::stop_net(&net),
        "net-undefine" => net_undefine::undefine_net(&net),
        "net-clean" => net_clean::clean_net(&net),
        "net-start" => net_start::start_net(&net),
        "net-uuid" => net_uuid::show_net_uuid(&net),
        "net-info" => net_info::show_net_info(&net),
        "net-dumpxml" => net_dumpxml::show_net_dumpxml(&net),
        _ => eprintln!("{} is not supported", cmd),
    }
    Ok(())
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
