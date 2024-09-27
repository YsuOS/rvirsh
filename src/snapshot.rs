use anyhow::{anyhow, bail, Context, Result};
use config::Config;
use std::env;
use virt::{domain::Domain, domain_snapshot::DomainSnapshot};

mod snapshot_create;
mod snapshot_current;
pub mod snapshot_delete;
mod snapshot_dumpxml;
mod snapshot_info;
mod snapshot_list;
mod snapshot_parent;
mod snapshot_revert;

use crate::{get_conn, get_domain};

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;
    let dom = get_domain(&conn, cmd)?;

    if cmd == "snapshot-delete" {
        snapshot_delete::delete_all_snapshots(&dom)?;
        return Ok(());
    } else if cmd == "snapshot-list" {
        snapshot_list::list_snapshots(&dom)?;
        return Ok(());
    } else if cmd == "snapshot-current" {
        snapshot_current::get_current_snapshot(&dom)?;
        return Ok(());
    } else if cmd == "snapshot-create" {
        // TODO: snapshot name can't be specified now
        snapshot_create::create_snapshot(&dom)?;
        return Ok(());
    }

    let snapshot = get_snapshot(&dom, cmd)?;

    match cmd {
        "snapshot-info" => snapshot_info::show_snapshot_info(&snapshot)?,
        "snapshot-revert" => snapshot_revert::revert_snapshot(&snapshot)?,
        "snapshot-parent" => snapshot_parent::show_snapshot_parent(&snapshot)?,
        "snapshot-dumpxml" => snapshot_dumpxml::show_snapshot_dumpxml(&snapshot)?,
        _ => bail!("{} is not supported", cmd),
    }
    Ok(())
}

fn get_snapshot_name(cmd: &str) -> Result<String> {
    let snapshot_name = env::args().nth(3).with_context(|| {
        anyhow!(
            "snapshot name is required\nUsage: rv {} <domain> <snapshot>",
            cmd
        )
    })?;
    Ok(snapshot_name)
}

fn get_snapshot(dom: &Domain, cmd: &str) -> Result<DomainSnapshot> {
    let snapshot_name = get_snapshot_name(cmd)?;
    Ok(DomainSnapshot::lookup_by_name(dom, &snapshot_name, 0)?)
}
