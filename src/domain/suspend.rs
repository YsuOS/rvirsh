use anyhow::Result;
use virt::domain::Domain;

pub fn suspend_domain(dom: &Domain) -> Result<()> {
    if !dom.is_active()? {
        crate::domain::error_domain_inactive(dom)?
    }
    dom.suspend()?;
    println!("Domain {} suspended", dom.get_name()?);
    Ok(())
}
