use anyhow::{anyhow, bail, Context, Result};
use config::Config;
use quick_xml::{events::Event, Reader};
use regex::Regex;
use std::{env, path::Path};
use virt::{
    connect::Connect,
    domain::Domain,
    storage_pool::StoragePool,
    storage_vol::StorageVol,
    sys::{VIR_DOMAIN_NONE, VIR_DOMAIN_PAUSED, VIR_DOMAIN_RUNNING, VIR_DOMAIN_SHUTOFF},
};

use crate::{err_msg, get_conn, get_domain, get_xml};

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    if cmd == "list" {
        list_domain(&conn)?;
        return Ok(());
    } else if cmd == "define" || cmd == "create" {
        let xml = get_xml(cmd)?;

        if cmd == "define" {
            define_domain(&conn, &xml)?;
        } else if cmd == "create" {
            create_domain(&conn, &xml)?;
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
        clone_domain(
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
        "dominfo" => show_domain_info(&dom)?,
        "domstate" => show_domain_state(&dom)?,
        "domid" => show_domain_id(&dom)?,
        "domuuid" => show_domain_uuid(&dom)?,
        "dumpxml" => show_domain_dumpxml(&dom)?,
        "info" => show_info(&dom)?,
        "start" => start_domain(&dom)?,
        "shutdown" => shutdown_domain(&dom)?,
        "reboot" => reboot_domain(&dom)?,
        "suspend" => suspend_domain(&dom)?,
        "resume" => resume_domain(&dom)?,
        "reset" => reset_domain(&dom)?,
        "poweroff" => poweroff_domain(&dom)?,
        "undefine" => undefine_domain(&dom)?,
        "autostart" => autostart_domain(&dom)?,
        "noautostart" => noautostart_domain(&dom)?,
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
pub fn autostart_domain(dom: &Domain) -> Result<()> {
    dom.set_autostart(true)?;
    println!("{} sets autostart", dom.get_name()?);
    Ok(())
}

pub fn clone_domain(
    conn: &Connect,
    org_name: &str,
    org_xml: &str,
    org_vol: &StorageVol,
    new_name: &str,
    new_pool: &StoragePool,
    new_vol_path: &str,
) -> Result<()> {
    let new_vol_xml = create_new_volume_xml(org_vol, new_vol_path)?;
    let new_xml = create_new_domain_xml(org_xml, org_vol, new_vol_path, org_name, new_name)?;

    crate::pool::pool_refresh::refresh_pool(new_pool)?;
    StorageVol::create_xml_from(new_pool, &new_vol_xml, org_vol, 0)?;

    let dom = Domain::define_xml(conn, &new_xml)?;
    println!("Domain {} is cloned from {}", dom.get_name()?, org_name);
    Ok(())
}

fn get_vol_name(vol_path: &str) -> Result<String> {
    let vol_name = Path::new(vol_path)
        .file_name()
        .and_then(|name| name.to_str())
        .context(anyhow!("Could not get volume name from volume path"))?;
    Ok(vol_name.to_string())
}

fn remove_uuid_lines(xml: &str) -> String {
    let re = Regex::new(r"(?m)^\s*<uuid>.*</uuid>\s*$").unwrap();
    re.replace_all(xml, "").to_string()
}

fn create_new_volume_xml(org_vol: &StorageVol, new_vol_path: &str) -> Result<String> {
    let vol_xml = org_vol.get_xml_desc(0)?;
    let updated_xml = vol_xml
        .replace(&org_vol.get_path()?, new_vol_path)
        .replace(&org_vol.get_name()?, &get_vol_name(new_vol_path)?);
    Ok(updated_xml)
}

fn create_new_domain_xml(
    org_xml: &str,
    org_vol: &StorageVol,
    new_vol_path: &str,
    org_name: &str,
    new_name: &str,
) -> Result<String> {
    let new_xml = org_xml
        .replace(org_name, new_name)
        .replace(&org_vol.get_path()?, new_vol_path);
    Ok(remove_uuid_lines(&new_xml))
}

pub fn create_domain(conn: &Connect, xml: &str) -> Result<()> {
    let dom = Domain::create_xml(conn, xml, VIR_DOMAIN_NONE)?;
    println!("Domain {} is created and running", dom.get_name()?);
    Ok(())
}
pub fn define_domain(conn: &Connect, xml: &str) -> Result<()> {
    let dom = Domain::define_xml(conn, xml)?;
    println!("Domain {} is defined", dom.get_name()?);
    Ok(())
}
pub fn show_domain_id(dom: &Domain) -> Result<()> {
    let id = crate::domain::get_id(dom)?;
    println!("{}", id);
    Ok(())
}

pub fn show_domain_info(dom: &Domain) -> Result<()> {
    let id = crate::domain::get_id(dom)?;
    let state = crate::domain::get_state_str(dom)?;
    let dominfo = dom.get_info()?;
    let persistent = if dom.is_persistent()? { "yes" } else { "no" };
    let autostart = if dom.get_autostart()? {
        "enable"
    } else {
        "disable"
    };
    let managed_save = if dom.has_managed_save(0)? {
        "yes"
    } else {
        "no"
    };

    println!("{:<20} {}", "Id:", id);
    println!("{:<20} {}", "Name:", dom.get_name()?);
    println!("{:<20} {}", "UUID:", dom.get_uuid_string()?);
    println!("{:<20} {}", "OS Type:", dom.get_os_type()?);
    println!("{:<20} {}", "State:", state);
    println!("{:<20} {}", "CPU(s):", dom.get_max_vcpus()?);
    println!("{:<20} {}", "CPU time:", dominfo.cpu_time);
    println!("{:<20} {} KiB", "Max memory:", dom.get_max_memory()?);
    println!("{:<20} {} KiB", "Used memory:", dominfo.memory);
    println!("{:<20} {}", "Persistent:", persistent);
    println!("{:<20} {}", "Autostart:", autostart);
    println!("{:<20} {}", "Managed save:", managed_save);
    // TODO: Add the following columns
    //println!("{:<20} {}", "Security model:", todo!());
    //println!("{:<20} {}", "Security DOI:", todo!());
    //println!("{:<20} {}", "Security label:", todo!());
    //println!("{:<20} {}", "Messages:", todo!());
    Ok(())
}
pub fn show_domain_state(dom: &Domain) -> Result<()> {
    let state = crate::domain::get_state_str(dom)?;
    println!("{}", state);
    Ok(())
}
pub fn show_domain_uuid(dom: &Domain) -> Result<()> {
    println!("{}", dom.get_uuid()?);
    Ok(())
}
pub fn show_domain_dumpxml(dom: &Domain) -> Result<()> {
    println!("{}", dom.get_xml_desc(0)?);
    Ok(())
}
pub fn show_info(dom: &Domain) -> Result<()> {
    show_domain_info(dom)?;
    //TODO: call another info command
    Ok(())
}
pub fn list_domain(conn: &Connect) -> Result<()> {
    let doms = conn.list_all_domains(0)?;

    println!("{:<20} {}", "Instance Name", "State");
    println!("{:<20} {}", "-".repeat(20), "-".repeat(10));
    for dom in doms {
        let name = dom.get_name()?;
        let state = crate::domain::get_state_str(&dom)?;
        println!("{:<20} {}", name, state);
    }
    Ok(())
}
pub fn noautostart_domain(dom: &Domain) -> Result<()> {
    dom.set_autostart(false)?;
    println!("{} unsets autostart", dom.get_name()?);
    Ok(())
}
pub fn poweroff_domain(dom: &Domain) -> Result<()> {
    if !dom.is_active()? {
        crate::domain::error_domain_inactive(dom)?
    }
    dom.destroy()?;
    println!("Domain {} is powered off", dom.get_name()?);
    Ok(())
}
pub fn reboot_domain(dom: &Domain) -> Result<()> {
    if !dom.is_active()? {
        crate::domain::error_domain_inactive(dom)?
    }
    dom.reboot(0)?;
    println!("Domain {} rebooted", dom.get_name()?);
    Ok(())
}
pub fn reset_domain(dom: &Domain) -> Result<()> {
    if !dom.is_active()? {
        crate::domain::error_domain_inactive(dom)?
    }
    dom.reset()?;
    println!("Domain {} is reset", dom.get_name()?);
    Ok(())
}
pub fn resume_domain(dom: &Domain) -> Result<()> {
    let (state, _) = dom.get_state()?;
    if state != VIR_DOMAIN_PAUSED {
        bail!("Domain {} is not paused", dom.get_name()?)
    }

    dom.resume()?;
    println!("Domain {} resumed", dom.get_name()?);
    Ok(())
}
pub fn shutdown_domain(dom: &Domain) -> Result<()> {
    if !dom.is_active()? {
        crate::domain::error_domain_inactive(dom)?
    }
    dom.shutdown()?;
    println!("Domain {} shutdowned", dom.get_name()?);
    Ok(())
}
pub fn start_domain(dom: &Domain) -> Result<()> {
    dom.create()?;
    println!("Domain {} started", dom.get_name()?);
    Ok(())
}
pub fn suspend_domain(dom: &Domain) -> Result<()> {
    if !dom.is_active()? {
        crate::domain::error_domain_inactive(dom)?
    }
    dom.suspend()?;
    println!("Domain {} suspended", dom.get_name()?);
    Ok(())
}
pub fn undefine_domain(dom: &Domain) -> Result<()> {
    dom.undefine()?;
    println!("Domain {} is undefined", dom.get_name()?);
    Ok(())
}
