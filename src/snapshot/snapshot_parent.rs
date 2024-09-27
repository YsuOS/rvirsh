use anyhow::Result;
use virt::domain_snapshot::DomainSnapshot;

pub fn show_snapshot_parent(snapshot: &DomainSnapshot) -> Result<()> {
    if let Ok(parent) = snapshot.get_parent(0) {
        println!("{}", parent.get_name()?);
    }
    Ok(())
}
