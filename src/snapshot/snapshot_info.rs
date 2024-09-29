use anyhow::Result;
use virt::{
    domain::Domain, domain_snapshot::DomainSnapshot, sys::VIR_DOMAIN_SNAPSHOT_LIST_DESCENDANTS,
};

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

fn get_parent(snapshot: &DomainSnapshot) -> Result<String> {
    if let Ok(parent) = snapshot.get_parent(0) {
        Ok(parent.get_name()?)
    } else {
        Ok("-".to_string())
    }
}
