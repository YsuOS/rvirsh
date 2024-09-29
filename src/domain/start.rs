use anyhow::Result;
use virt::domain::Domain;

pub fn create_domain(dom: &Domain) -> Result<()> {
    dom.create()?;
    println!("Domain {} started", dom.get_name()?);
    Ok(())
}
