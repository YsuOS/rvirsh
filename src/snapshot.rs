use anyhow::Result;
use config::Config;
use std::env;
use virt::domain_snapshot::DomainSnapshot;

mod snapshot_create;
mod snapshot_current;
pub mod snapshot_delete;
mod snapshot_dumpxml;
mod snapshot_info;
mod snapshot_list;
mod snapshot_parent;
mod snapshot_revert;

use crate::{get_conn, get_domain, help::help_snapshot};

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;
    let dom = get_domain(&conn, cmd)?;

    if cmd == "snapshot-delete" {
        snapshot_delete::delete_all_snapshots(&dom);
        return Ok(());
    } else if cmd == "snapshot-list" {
        snapshot_list::list_snapshots(&dom);
        return Ok(());
    } else if cmd == "snapshot-current" {
        snapshot_current::get_current_snapshot(&dom);
        return Ok(());
    } else if cmd == "snapshot-create" {
        // TODO: snapshot name can be specified now
        snapshot_create::create_snapshot(&dom);
        return Ok(());
    }

    let snapshot_name = env::args().nth(3);
    if snapshot_name.is_none() {
        help_snapshot(cmd);
        return Ok(());
    }

    let snapshot_name = snapshot_name.unwrap();
    let snapshot = DomainSnapshot::lookup_by_name(&dom, &snapshot_name, 0).unwrap();

    // snapshot-(parent|dumpxml) does not require Domain, but it is needed to specify the snapshot
    match cmd {
        "snapshot-info" => snapshot_info::show_snapshot_info(&dom, &snapshot),
        "snapshot-revert" => snapshot_revert::revert_snapshot(&dom, &snapshot),
        "snapshot-parent" => snapshot_parent::show_snapshot_parent(&snapshot),
        "snapshot-dumpxml" => snapshot_dumpxml::show_snapshot_dumpxml(&snapshot),
        _ => eprintln!("{} is not supported", cmd),
    }
    Ok(())
}
