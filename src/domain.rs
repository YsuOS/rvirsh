mod autostart;
mod clone;
pub mod create;
pub mod define;
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

use std::env;

use anyhow::{anyhow, bail, Context, Result};
use config::Config;
use quick_xml::{events::Event, Reader};
use virt::{
    domain::Domain,
    storage_pool::StoragePool,
    storage_vol::StorageVol,
    sys::{VIR_DOMAIN_PAUSED, VIR_DOMAIN_RUNNING, VIR_DOMAIN_SHUTOFF},
};

use crate::{err_msg, get_conn, get_domain, get_xml};

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    if cmd == "list" {
        list::list_domain(&conn)?;
        return Ok(());
    } else if cmd == "define" || cmd == "create" {
        let xml = get_xml(cmd)?;

        if cmd == "define" {
            define::define_domain(&conn, &xml)?;
        } else if cmd == "create" {
            create::create_domain(&conn, &xml)?;
        }
        return Ok(());
    }

    let dom = get_domain(&conn, cmd)?;

    if cmd == "clone" {
        let new_name = env::args().nth(3).with_context(|| {
            err_msg(
                "New domain name is required",
                cmd,
                vec!["<org domain>", "<new domain>"],
            )
        })?;
        let new_pool_name = settings.get_string("POOL")?;
        let new_pool = StoragePool::lookup_by_name(&conn, &new_pool_name)?;
        let new_vol_path = env::args().nth(4).with_context(|| {
            err_msg(
                "New volume path is required",
                cmd,
                vec!["<org domain>", "<new domain>", "<new volume path>"],
            )
        })?;
        clone::clone_domain(
            &conn,
            &dom.get_name()?,
            &dom.get_xml_desc(0)?,
            &get_volume_from_domain(&dom)?,
            &new_name,
            &new_pool,
            &new_vol_path,
        )?;
        return Ok(());
    }

    match cmd {
        "dominfo" => dominfo::show_domain_info(&dom)?,
        "domstate" => domstate::show_domain_state(&dom)?,
        "domid" => domid::show_domain_id(&dom)?,
        "domuuid" => domuuid::show_domain_uuid(&dom)?,
        "dumpxml" => dumpxml::show_domain_dumpxml(&dom)?,
        "info" => info::show_info(&dom)?,
        "start" => start::start_domain(&dom)?,
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

fn get_id(dom: &Domain) -> Result<String> {
    Ok(dom.get_id().map_or("-".to_string(), |id| id.to_string()))
}

fn error_domain_inactive(dom: &Domain) -> Result<()> {
    bail!("Domain {} is inactive", dom.get_name()?)
}

fn get_volume_from_domain(dom: &Domain) -> Result<StorageVol> {
    let xml = dom.get_xml_desc(0)?;
    let mut reader = Reader::from_str(&xml);
    let mut file_path = None;

    loop {
        match reader.read_event() {
            Ok(Event::Empty(e)) if e.name().as_ref() == b"source" => {
                if let Some(attr) = e
                    .attributes()
                    .find(|a| a.as_ref().unwrap().key.as_ref() == b"file")
                {
                    file_path = Some(attr?.unescape_value()?.to_string());
                }
                break;
            }
            Err(e) => panic!("Error: {}", e),
            Ok(Event::Eof) => break,
            _ => (),
        }
    }
    let file_path = file_path.with_context(|| {
        anyhow!(
            "Could not find volume from {}'s xml",
            dom.get_name().unwrap()
        )
    })?;
    Ok(StorageVol::lookup_by_path(&dom.get_connect()?, &file_path)?)
}
