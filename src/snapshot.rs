use crate::{
    get_conn, get_domain,
    Commands::{self, *},
};
use anyhow::Result;
use config::Config;
use virt::{
    domain::Domain,
    domain_snapshot::DomainSnapshot,
    sys::{
        VIR_DOMAIN_SNAPSHOT_CREATE_CURRENT, VIR_DOMAIN_SNAPSHOT_CREATE_REDEFINE,
        VIR_DOMAIN_SNAPSHOT_LIST_DESCENDANTS, VIR_DOMAIN_SNAPSHOT_REVERT_RUNNING,
    },
};

//fn get_snapshot_name(dom: &Domain) -> Result<String> {
//    Ok(get_args(
//        3,
//        "snapshot name is required",
//        cmd,
//        &vec!["<domain>", "<snapshot>"],
//    )?)
//}

fn get_snapshot(dom: &Domain, ss: &str) -> Result<DomainSnapshot> {
    Ok(DomainSnapshot::lookup_by_name(dom, ss, 0)?)
}

fn get_parent(snapshot: &DomainSnapshot) -> Result<String> {
    if let Ok(parent) = snapshot.get_parent(0) {
        Ok(parent.get_name()?)
    } else {
        Ok("-".to_string())
    }
}

/// `rv snapshot-list`
pub fn list_snapshots(dom: &Domain) -> Result<()> {
    let snapshots = dom.list_all_snapshots(0)?;

    //TODO: Add "Creation Time", "State" column
    println!("{:<10}", "Name");
    println!("{:<10}", "-".repeat(10));
    for snapshot in snapshots {
        let name = snapshot.get_name()?;
        println!("{:<10}", name);
    }
    Ok(())
}

/// `rv snapshot-parent`
pub fn show_snapshot_parent(snapshot: &DomainSnapshot) -> Result<()> {
    if let Ok(parent) = snapshot.get_parent(0) {
        println!("{}", parent.get_name()?);
    }
    Ok(())
}

/// `rv snapshot-revert`
pub fn revert_snapshot(dom: &Domain, snapshot: &DomainSnapshot) -> Result<()> {
    let xml = snapshot.get_xml_desc(0)?;
    DomainSnapshot::create_xml(
        dom,
        &xml,
        VIR_DOMAIN_SNAPSHOT_CREATE_REDEFINE | VIR_DOMAIN_SNAPSHOT_CREATE_CURRENT,
    )?;

    snapshot.revert(VIR_DOMAIN_SNAPSHOT_REVERT_RUNNING)?;

    println!("{} is set on {}", snapshot.get_name()?, dom.get_name()?);
    Ok(())
}

/// `rv snapshot-create`
pub fn create_snapshot(dom: &Domain, snapshot_name: &str) -> Result<()> {
    let xml = "<domainsnapshot><name>".to_string() + snapshot_name + "</name></domainsnapshot>";
    let snapshot = DomainSnapshot::create_xml(dom, &xml, 0)?;

    println!("{} is created on {}", snapshot.get_name()?, dom.get_name()?);
    Ok(())
}

/// `rv snaphost-current`
pub fn get_current_snapshot(dom: &Domain) -> Result<()> {
    let snapshot = DomainSnapshot::current(dom, 0)?;

    println!("{}", snapshot.get_name()?);
    Ok(())
}

/// `rv snapshot-delete`
pub fn delete_all_snapshots(dom: &Domain) -> Result<()> {
    let snapshots = dom.list_all_snapshots(0)?;

    for snapshot in snapshots {
        snapshot.delete(0)?;
        println!("{} is deleted", snapshot.get_name()?);
    }
    println!("All snapshots of {} is deleted", dom.get_name()?);
    Ok(())
}

/// `rv snapshot-dumpxml`
pub fn show_snapshot_dumpxml(snapshot: &DomainSnapshot) -> Result<()> {
    println!("{}", snapshot.get_xml_desc(0)?);
    Ok(())
}

/// `rv snapshot-info`
pub fn show_snapshot_info(domain: &Domain, snapshot: &DomainSnapshot) -> Result<()> {
    let current = if snapshot.is_current(0)? { "yes" } else { "no" };
    let metadata = if snapshot.has_metadata(0)? {
        "yes"
    } else {
        "no"
    };
    let parent_name = get_parent(snapshot)?;

    println!("{:<15} {}", "Name:", snapshot.get_name()?);
    println!("{:<15} {}", "Domain:", domain.get_name()?);
    println!("{:<15} {}", "Current:", current);
    //println!("{:<15} {}", "State:", todo!());
    //println!("{:<15} {}", "Location:", todo!());
    println!("{:<15} {}", "Parent:", parent_name);
    println!("{:<15} {}", "Children:", snapshot.num_children(0)?);
    println!(
        "{:<15} {}",
        "Descendants:",
        snapshot.num_children(VIR_DOMAIN_SNAPSHOT_LIST_DESCENDANTS)?
    );
    println!("{:<15} {}", "Metadata:", metadata);
    Ok(())
}

pub fn main(settings: &Config, cmd: &Commands) -> Result<()> {
    match cmd {
        SnapshotDelete(dom) | SnapshotList(dom) | SnapshotCurrent(dom) => {
            let dom = get_domain(&get_conn(settings)?, &dom.name)?;
            match cmd {
                SnapshotDelete(_) => delete_all_snapshots(&dom)?,
                SnapshotList(_) => list_snapshots(&dom)?,
                SnapshotCurrent(_) => get_current_snapshot(&dom)?,
                _ => unreachable!(),
            }
        }
        SnapshotCreate(args)
        | SnapshotInfo(args)
        | SnapshotRevert(args)
        | SnapshotParent(args)
        | SnapshotDumpxml(args) => {
            let dom = get_domain(&get_conn(settings)?, &args.dom)?;
            if let SnapshotCreate(args) = cmd {
                create_snapshot(&dom, &args.ss)?;
                return Ok(());
            }

            let snapshot = get_snapshot(&dom, &args.ss)?;
            match cmd {
                SnapshotInfo(_) => show_snapshot_info(&dom, &snapshot)?,
                SnapshotRevert(_) => revert_snapshot(&dom, &snapshot)?,
                SnapshotParent(_) => show_snapshot_parent(&snapshot)?,
                SnapshotDumpxml(_) => show_snapshot_dumpxml(&snapshot)?,
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
