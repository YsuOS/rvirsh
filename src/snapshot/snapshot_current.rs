use anyhow::Result;
use virt::{domain::Domain, domain_snapshot::DomainSnapshot};

pub fn get_current_snapshot(dom: &Domain) -> Result<()> {
    let snapshot = DomainSnapshot::current(dom, 0)?;

    println!("{}", snapshot.get_name()?);
    Ok(())
}
