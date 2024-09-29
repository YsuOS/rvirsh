use anyhow::Result;
use virt::domain::Domain;

pub fn undefine_domain(dom: &Domain) -> Result<()> {
    dom.undefine()?;
    println!("Domain {} is undefined", dom.get_name()?);
    Ok(())
}
