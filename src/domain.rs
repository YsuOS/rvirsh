use crate::{
    get_conn, get_domain, get_xml,
    Commands::{self, *},
};
use anyhow::{bail, ensure, Result};
use config::Config;
use virt::{
    connect::Connect,
    domain::Domain,
    sys::{VIR_DOMAIN_NONE, VIR_DOMAIN_PAUSED, VIR_DOMAIN_RUNNING, VIR_DOMAIN_SHUTOFF},
};

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

fn ensure_domain_active(dom: &Domain) -> Result<()> {
    ensure!(
        dom.is_active()?,
        format!("Domain {} is inactive", dom.get_name()?)
    );
    Ok(())
}

/// `rv list`
pub fn list_domain(conn: &Connect) -> Result<()> {
    let doms = conn.list_all_domains(0)?;

    println!("{:<20} {}", "Instance Name", "State");
    println!("{:<20} {}", "-".repeat(20), "-".repeat(10));
    for dom in doms {
        let name = dom.get_name()?;
        let state = get_state_str(&dom)?;
        println!("{:<20} {}", name, state);
    }
    Ok(())
}

/// `rv define`
pub fn define_domain(conn: &Connect, xml: &str) -> Result<()> {
    let dom = Domain::define_xml(conn, xml)?;
    println!("Domain {} is defined", dom.get_name()?);
    Ok(())
}

/// `rv create`
pub fn create_domain(conn: &Connect, xml: &str) -> Result<()> {
    let dom = Domain::create_xml(conn, xml, VIR_DOMAIN_NONE)?;
    println!("Domain {} is created and running", dom.get_name()?);
    Ok(())
}

/// `rv domid`
pub fn show_domain_id(dom: &Domain) -> Result<()> {
    println!("{}", get_id(dom)?);
    Ok(())
}

/// `rv dominfo`
pub fn show_domain_info(dom: &Domain) -> Result<()> {
    let id = get_id(dom)?;
    let state = get_state_str(dom)?;
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

/// `rv domstate`
pub fn show_domain_state(dom: &Domain) -> Result<()> {
    let state = get_state_str(dom)?;
    println!("{}", state);
    Ok(())
}

/// `rv domuuid`
pub fn show_domain_uuid(dom: &Domain) -> Result<()> {
    println!("{}", dom.get_uuid()?);
    Ok(())
}

/// `rv dumpxml`
pub fn show_domain_dumpxml(dom: &Domain) -> Result<()> {
    println!("{}", dom.get_xml_desc(0)?);
    Ok(())
}

/// `rv info`
pub fn show_info(dom: &Domain) -> Result<()> {
    show_domain_info(dom)?;
    //TODO: call another info command
    Ok(())
}

/// `rv noautostart`
pub fn noautostart_domain(dom: &Domain) -> Result<()> {
    dom.set_autostart(false)?;
    println!("{} unsets autostart", dom.get_name()?);
    Ok(())
}

/// `rv autostart`
pub fn autostart_domain(dom: &Domain) -> Result<()> {
    dom.set_autostart(true)?;
    println!("{} sets autostart", dom.get_name()?);
    Ok(())
}

/// `rv poweroff`
pub fn poweroff_domain(dom: &Domain) -> Result<()> {
    ensure_domain_active(dom)?;
    dom.destroy()?;
    println!("Domain {} is powered off", dom.get_name()?);
    Ok(())
}

/// `rv reboot`
pub fn reboot_domain(dom: &Domain) -> Result<()> {
    ensure_domain_active(dom)?;
    dom.reboot(0)?;
    println!("Domain {} rebooted", dom.get_name()?);
    Ok(())
}

/// `rv reset`
pub fn reset_domain(dom: &Domain) -> Result<()> {
    ensure_domain_active(dom)?;
    dom.reset()?;
    println!("Domain {} is reset", dom.get_name()?);
    Ok(())
}

/// `rv resume`
pub fn resume_domain(dom: &Domain) -> Result<()> {
    let (state, _) = dom.get_state()?;
    if state != VIR_DOMAIN_PAUSED {
        bail!("Domain {} is not paused", dom.get_name()?)
    }

    dom.resume()?;
    println!("Domain {} resumed", dom.get_name()?);
    Ok(())
}

/// `rv shutdown`
pub fn shutdown_domain(dom: &Domain) -> Result<()> {
    ensure_domain_active(dom)?;
    dom.shutdown()?;
    println!("Domain {} shutdowned", dom.get_name()?);
    Ok(())
}

/// `rv start`
pub fn start_domain(dom: &Domain) -> Result<()> {
    dom.create()?;
    println!("Domain {} started", dom.get_name()?);
    Ok(())
}

/// `rv suspend`
pub fn suspend_domain(dom: &Domain) -> Result<()> {
    ensure_domain_active(dom)?;
    dom.suspend()?;
    println!("Domain {} suspended", dom.get_name()?);
    Ok(())
}

/// `rv undefine`
pub fn undefine_domain(dom: &Domain) -> Result<()> {
    dom.undefine()?;
    println!("Domain {} is undefined", dom.get_name()?);
    Ok(())
}

pub fn main(settings: &Config, cmd: &Commands) -> Result<()> {
    let conn = get_conn(settings)?;

    match cmd {
        List => {
            list_domain(&conn)?;
            return Ok(());
        }
        Define(xml) | Create(xml) => {
            let xml = get_xml(&xml.name)?;

            match cmd {
                Define(_) => define_domain(&conn, &xml)?,
                Create(_) => create_domain(&conn, &xml)?,
                _ => unreachable!(),
            }
            return Ok(());
        }
        Start(dom) | Shutdown(dom) | Reboot(dom) | Suspend(dom) | Resume(dom) | Reset(dom)
        | Poweroff(dom) | Undefine(dom) | Dominfo(dom) | Info(dom) | Domid(dom) | Domuuid(dom)
        | Autostart(dom) | Noautostart(dom) | Domstate(dom) | Dumpxml(dom) => {
            let dom = get_domain(&conn, &dom.name)?;

            match cmd {
                Dominfo(_) => show_domain_info(&dom)?,
                Domstate(_) => show_domain_state(&dom)?,
                Domid(_) => show_domain_id(&dom)?,
                Domuuid(_) => show_domain_uuid(&dom)?,
                Dumpxml(_) => show_domain_dumpxml(&dom)?,
                Info(_) => show_info(&dom)?,
                Start(_) => start_domain(&dom)?,
                Shutdown(_) => shutdown_domain(&dom)?,
                Reboot(_) => reboot_domain(&dom)?,
                Suspend(_) => suspend_domain(&dom)?,
                Resume(_) => resume_domain(&dom)?,
                Reset(_) => reset_domain(&dom)?,
                Poweroff(_) => poweroff_domain(&dom)?,
                Undefine(_) => undefine_domain(&dom)?,
                Autostart(_) => autostart_domain(&dom)?,
                Noautostart(_) => noautostart_domain(&dom)?,
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
