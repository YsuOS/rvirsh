use anyhow::Result;
use virt::{domain::Domain, domain_snapshot::DomainSnapshot};

pub fn create_snapshot(dom: &Domain, snapshot_name: &str) -> Result<()> {
    let xml = "<domainsnapshot><name>".to_string() + snapshot_name + "</name></domainsnapshot>";
    let snapshot = DomainSnapshot::create_xml(dom, &xml, 0)?;

    println!("{} is created on {}", snapshot.get_name()?, dom.get_name()?);
    Ok(())
}
