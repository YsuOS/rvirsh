mod autostart;
mod create;
mod define;
mod domid;
mod dominfo;
mod domstate;
mod domuuid;
mod dumpxml;
mod info;
pub mod list;
mod noautostart;
pub mod poweroff;
mod reboot;
mod reset;
mod resume;
mod shutdown;
pub mod start;
mod suspend;
pub mod undefine;

use anyhow::{bail, Context, Result};
use config::Config;
use virt::{
    domain::Domain,
    sys::{VIR_DOMAIN_PAUSED, VIR_DOMAIN_RUNNING, VIR_DOMAIN_SHUTOFF},
};

use crate::{get_conn, get_domain, get_xml};

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    if cmd == "list" {
        list::list_domain(&conn)?;
        return Ok(());
    } else if cmd == "define" || cmd == "create" {
        let mut xml = get_xml(cmd)?;

        if cmd == "define" {
            define::define_domain(&conn, &mut xml)?;
        } else if cmd == "create" {
            create::create_domain(&conn, &mut xml)?;
        }
        return Ok(());
    }

    let dom = get_domain(&conn, cmd)?;

    match cmd {
        "dominfo" => dominfo::show_domain_info(&dom)?,
        "domstate" => domstate::show_domain_state(&dom)?,
        "domid" => domid::show_domain_id(&dom)?,
        "domuuid" => domuuid::show_domain_uuid(&dom)?,
        "dumpxml" => dumpxml::show_domain_dumpxml(&dom)?,
        "info" => info::show_info(&dom)?,
        "start" => start::create_domain(&dom)?,
        "shutdown" => shutdown::shutdown_domain(&dom)?,
        "reboot" => reboot::reboot_domain(&dom)?,
        "suspend" => suspend::suspend_domain(&dom)?,
        "resume" => resume::resume_domain(&dom)?,
        "reset" => reset::reset_domain(&dom)?,
        "poweroff" => poweroff::poweroff_domain(&dom)?,
        "undefine" => undefine::undefine_domain(&dom)?,
        "autostart" => autostart::autostart_domain(&dom)?,
        "noautostart" => noautostart::noautostart_domain(&dom)?,
        _ => bail!("{} is not supported", cmd),
    }
    Ok(())
}

fn get_state_str(dom: &Domain) -> Result<&str> {
    let state = match dom.get_state()? {
        (VIR_DOMAIN_RUNNING, _) => "running",
        (VIR_DOMAIN_PAUSED, _) => "paused",
        (VIR_DOMAIN_SHUTOFF, _) => "shut off",
        // TODO: support other states on demand
        _ => "-",
    };
    Ok(state)
}

fn get_id(dom: &Domain) -> Result<u32> {
    let id = dom.get_id().context("Cannot find ID")?;
    Ok(id)
}

fn error_domain_inactive(dom: &Domain) -> Result<()> {
    bail!("Domain {} is inactive", dom.get_name()?)
}
