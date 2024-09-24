use anyhow::Result;
use virt::domain::Domain;

pub fn show_domain_id(dom: &Domain) -> Result<()> {
    let id = crate::domain::get_id(dom)?;
    println!("{}", id);
    Ok(())
}
