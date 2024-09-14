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
mod shutdown;
mod start;
mod suspend;
pub mod undefine;

use config::Config;
use std::{env, fs::File};
use virt::{
    connect::Connect,
    domain::Domain,
    sys::{VIR_DOMAIN_PAUSED, VIR_DOMAIN_RUNNING, VIR_DOMAIN_SHUTOFF},
};

use crate::help::{help_domain, help_domain_xml};

pub fn main(settings: &Config, cmd: &str) {
    let uri = settings.get_string("URI").unwrap();
    let conn = Connect::open(Some(&uri)).unwrap();

    if cmd == "list" {
        list::list_domain(&conn);
        return;
    } else if cmd == "define" {
        let xml_path = env::args().nth(2);
        if xml_path.is_none() {
            help_domain_xml(cmd);
            return;
        }
        let mut xml = File::open(xml_path.unwrap()).unwrap();

        define::define_domain(&conn, &mut xml);
        return;
    }

    let dom_name = env::args().nth(2);
    if dom_name.is_none() {
        help_domain(cmd);
        return;
    }

    let dom_name = dom_name.unwrap();
    let dom = Domain::lookup_by_name(&conn, &dom_name).unwrap();

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
