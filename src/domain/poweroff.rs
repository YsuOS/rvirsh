use anyhow::Result;
use virt::domain::Domain;

pub fn poweroff_domain(dom: &Domain) -> Result<()> {
    if !dom.is_active()? {
        crate::domain::error_domain_inactive(dom)?
    }
    dom.destroy()?;
    println!("Domain {} is powered off", dom.get_name()?);
    Ok(())
}
