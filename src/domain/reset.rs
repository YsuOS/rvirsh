use anyhow::Result;
use virt::domain::Domain;

pub fn reset_domain(dom: &Domain) -> Result<()> {
    if !dom.is_active()? {
        crate::domain::error_domain_inactive(dom)?
    }
    dom.reset()?;
    println!("Domain {} is reset", dom.get_name()?);
    Ok(())
}
