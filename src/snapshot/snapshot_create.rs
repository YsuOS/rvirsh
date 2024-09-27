use anyhow::Result;
use virt::{domain::Domain, domain_snapshot::DomainSnapshot};

pub fn create_snapshot(dom: &Domain) -> Result<()> {
    let xml = "<domainsnapshot/>";
    let snapshot = DomainSnapshot::create_xml(dom, xml, 0)?;

    println!("{} is created on {}", snapshot.get_name()?, dom.get_name()?);
    Ok(())
}
