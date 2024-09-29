use anyhow::Result;
use virt::domain::Domain;

pub fn show_domain_uuid(dom: &Domain) -> Result<()> {
    println!("{}", dom.get_uuid()?);
    Ok(())
}
