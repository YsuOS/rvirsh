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

use anyhow::{anyhow, bail, Context, Result};
use config::Config;
use virt::{connect::Connect, network::Network};

use crate::{get_conn, get_xml};

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    if cmd == "net-list" {
        net_list::list_net(&conn)?;
        return Ok(());
    } else if cmd == "net-define" || cmd == "net-create" {
        let mut xml = get_xml(cmd)?;

        if cmd == "net-define" {
            net_define::define_net(&conn, &mut xml)?;
        } else if cmd == "net-create" {
            net_create::create_net(&conn, &mut xml)?;
        }
        return Ok(());
    }

    let net = get_network(&conn, cmd)?;

    match cmd {
        "net-autostart" => net_autostart::autostart_net(&net)?,
        "net-noautostart" => net_noautostart::noautostart_net(&net)?,
        "net-stop" => net_stop::stop_net(&net)?,
        "net-undefine" => net_undefine::undefine_net(&net)?,
        "net-clean" => net_clean::clean_net(&net)?,
        "net-start" => net_start::start_net(&net)?,
        "net-uuid" => net_uuid::show_net_uuid(&net)?,
        "net-info" => net_info::show_net_info(&net)?,
        "net-dumpxml" => net_dumpxml::show_net_dumpxml(&net)?,
        _ => bail!("{} is not supported", cmd),
    }
    Ok(())
}

fn get_net_name(cmd: &str) -> Result<String> {
    let net_name = env::args()
        .nth(2)
        .with_context(|| anyhow!("Network name is required\nUsage: rv {} <network>", cmd))?;
    Ok(net_name)
}

fn get_network(conn: &Connect, cmd: &str) -> Result<Network> {
    let net_name = get_net_name(cmd)?;
    Ok(Network::lookup_by_name(conn, &net_name)?)
}

fn get_autostart_str(net: &Network) -> Result<&str> {
    let ret = if net.get_autostart()? { "yes" } else { "no" };
    Ok(ret)
}

fn get_persistent_str(net: &Network) -> Result<&str> {
    let ret = if net.is_persistent()? { "yes" } else { "no" };
    Ok(ret)
}
