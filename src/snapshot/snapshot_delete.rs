use anyhow::Result;
use virt::domain::Domain;

pub fn delete_all_snapshots(dom: &Domain) -> Result<()> {
    let snapshots = dom.list_all_snapshots(0)?;

    for snapshot in snapshots {
        snapshot.delete(0)?;
        println!("{} is deleted", snapshot.get_name()?);
    }
    println!("All snapshots of {} is deleted", dom.get_name()?);
    Ok(())
}
