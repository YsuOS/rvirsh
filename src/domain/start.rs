use anyhow::Result;
use virt::domain::Domain;

pub fn start_domain(dom: &Domain) -> Result<()> {
    dom.create()?;
    println!("Domain {} started", dom.get_name()?);
    Ok(())
}
