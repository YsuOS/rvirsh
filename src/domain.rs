mod autostart;
mod define;
mod domid;
mod dominfo;
mod domstate;
mod domuuid;
mod dumpxml;
mod info;
mod list;
mod noautostart;
mod poweroff;
mod reboot;
mod reset;
mod resume;
mod run;
mod shutdown;
mod start;
mod suspend;
pub mod undefine;

use anyhow::Result;
use config::Config;
use virt::{
    domain::Domain,
    sys::{VIR_DOMAIN_PAUSED, VIR_DOMAIN_RUNNING, VIR_DOMAIN_SHUTOFF},
};

use crate::{get_conn, get_domain, get_xml};

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    if cmd == "list" {
        list::list_domain(&conn);
        return Ok(());
    } else if cmd == "define" || cmd == "run" {
        let mut xml = get_xml(cmd)?;

        if cmd == "define" {
            define::define_domain(&conn, &mut xml);
        } else if cmd == "run" {
            run::run_domain(&conn, &mut xml);
        }
        return Ok(());
    }

    let dom = get_domain(&conn, cmd)?;

    match cmd {
        "dominfo" => dominfo::show_domain_info(&dom),
        "domstate" => domstate::show_domain_state(&dom),
        "domid" => domid::show_domain_id(&dom),
        "domuuid" => domuuid::show_domain_uuid(&dom),
        "dumpxml" => dumpxml::show_domain_dumpxml(&dom),
        "info" => info::show_domain_info(&dom),
        "start" => start::create_domain(&dom),
        "shutdown" => shutdown::shutdown_domain(&dom),
        "reboot" => reboot::reboot_domain(&dom),
        "suspend" => suspend::suspend_domain(&dom),
        "resume" => resume::resume_domain(&dom),
        "reset" => reset::reset_domain(&dom),
        "poweroff" => poweroff::poweroff_domain(&dom),
        "undefine" => undefine::undefine_domain(&dom),
        "autostart" => autostart::autostart_domain(&dom),
        "noautostart" => noautostart::noautostart_domain(&dom),
        _ => eprintln!("{} is not supported", cmd),
    }
    Ok(())
}

pub fn get_state_str(dom: &Domain) -> &str {
    match dom.get_state().unwrap() {
        (VIR_DOMAIN_RUNNING, _) => "running",
        (VIR_DOMAIN_PAUSED, _) => "paused",
        (VIR_DOMAIN_SHUTOFF, _) => "shut off",
        // TODO: support other states on demand
        _ => "-",
    }
}
