use anyhow::Result;
use virt::domain::Domain;

pub fn show_info(dom: &Domain) -> Result<()> {
    crate::domain::dominfo::show_domain_info(dom)?;
    //TODO: call another info command
    Ok(())
}
