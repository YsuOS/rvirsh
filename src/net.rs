use crate::{get_args, get_conn, get_xml};
use anyhow::{bail, Result};
use config::Config;
use virt::{connect::Connect, network::Network};

fn get_network(conn: &Connect, cmd: &str) -> Result<Network> {
    let name = get_args(2, "Network name is required", cmd, &vec!["<network>"])?;
    Ok(Network::lookup_by_name(conn, &name)?)
}

fn get_autostart_str(net: &Network) -> Result<&str> {
    Ok(if net.get_autostart()? { "yes" } else { "no" })
}

fn get_persistent_str(net: &Network) -> Result<&str> {
    Ok(if net.is_persistent()? { "yes" } else { "no" })
}

/// `rv net-clean`
pub fn clean_net(net: &Network) -> Result<()> {
    if net.is_active()? {
        stop_net(net)?;
    }

    undefine_net(net)?;
    Ok(())
}

/// `rv net-create`
pub fn create_net(conn: &Connect, xml: &str) -> Result<()> {
    let net = Network::create_xml(conn, xml)?;
    println!("{} is created", net.get_name()?);
    Ok(())
}

/// `rv net-define`
pub fn define_net(conn: &Connect, xml: &str) -> Result<()> {
    let net = Network::define_xml(conn, xml)?;
    println!("{} is defined", net.get_name()?);
    Ok(())
}

/// `rv net-dumpxml`
pub fn show_net_dumpxml(net: &Network) -> Result<()> {
    println!("{}", net.get_xml_desc(0)?);
    Ok(())
}

/// `rv net-info`
pub fn show_net_info(net: &Network) -> Result<()> {
    let active = if net.is_active()? { "yes" } else { "no" };
    let persistent = get_persistent_str(net)?;
    let autostart = get_autostart_str(net)?;

    println!("{:<15} {}", "Name:", net.get_name()?);
    println!("{:<15} {}", "UUID:", net.get_uuid()?);
    println!("{:<15} {}", "Active:", active);
    println!("{:<15} {}", "Persistent:", persistent);
    println!("{:<15} {}", "Autostart:", autostart);
    println!("{:<15} {}", "Bridge:", net.get_bridge_name()?);
    Ok(())
}

/// `rv net-list`
pub fn list_net(conn: &Connect) -> Result<()> {
    let networks = conn.list_all_networks(0)?;

    println!(
        "{:<10} {:<8} {:<10} {}",
        "Name", "State", "Autostart", "Persistent"
    );
    println!(
        "{:<10} {:<8} {:<10} {}",
        "-".repeat(10),
        "-".repeat(8),
        "-".repeat(10),
        "-".repeat(12)
    );
    for net in networks {
        let name = net.get_name()?;
        let state = if net.is_active()? {
            "active"
        } else {
            "inactive"
        };
        let autostart = get_autostart_str(&net)?;
        let persistent = get_persistent_str(&net)?;
        println!("{:<10} {:<8} {:<10} {}", name, state, autostart, persistent);
    }
    Ok(())
}

/// `rv net-autostart`
pub fn autostart_net(net: &Network) -> Result<()> {
    net.set_autostart(true)?;
    println!("{} sets autostart", net.get_name()?);
    Ok(())
}

/// `rv net-noautostart`
pub fn noautostart_net(net: &Network) -> Result<()> {
    net.set_autostart(false)?;
    println!("{} unsets autostart", net.get_name()?);
    Ok(())
}

/// `rv net-start`
pub fn start_net(net: &Network) -> Result<()> {
    if net.is_active()? {
        bail!("{} already started", net.get_name()?)
    }

    net.create()?;
    println!("{} started", net.get_name()?);
    Ok(())
}

/// `rv net-stop`
pub fn stop_net(net: &Network) -> Result<()> {
    if !net.is_active()? {
        bail!("{} is inactive", net.get_name()?)
    }

    net.destroy()?;
    println!("{} is stopped", net.get_name()?);
    Ok(())
}

/// `rv net-undefine`
pub fn undefine_net(net: &Network) -> Result<()> {
    net.undefine()?;
    println!("{} is undefined", net.get_name()?);
    Ok(())
}

/// `rv net-uuid`
pub fn show_net_uuid(net: &Network) -> Result<()> {
    println!("{}", net.get_uuid()?);
    Ok(())
}

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    if cmd == "net-list" {
        list_net(&conn)?;
        return Ok(());
    } else if cmd == "net-define" || cmd == "net-create" {
        let xml = get_xml(cmd)?;

        if cmd == "net-define" {
            define_net(&conn, &xml)?;
        } else if cmd == "net-create" {
            create_net(&conn, &xml)?;
        }
        return Ok(());
    }

    let net = get_network(&conn, cmd)?;

    match cmd {
        "net-autostart" => autostart_net(&net)?,
        "net-noautostart" => noautostart_net(&net)?,
        "net-stop" => stop_net(&net)?,
        "net-undefine" => undefine_net(&net)?,
        "net-clean" => clean_net(&net)?,
        "net-start" => start_net(&net)?,
        "net-uuid" => show_net_uuid(&net)?,
        "net-info" => show_net_info(&net)?,
        "net-dumpxml" => show_net_dumpxml(&net)?,
        _ => bail!("{} is not supported", cmd),
    }
    Ok(())
}
