use anyhow::Result;
use virt::domain::Domain;

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
