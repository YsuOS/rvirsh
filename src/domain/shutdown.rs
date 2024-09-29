use anyhow::Result;
use virt::domain::Domain;

pub fn shutdown_domain(dom: &Domain) -> Result<()> {
    if !dom.is_active()? {
        crate::domain::error_domain_inactive(dom)?
    }
    dom.shutdown()?;
    println!("Domain {} shutdowned", dom.get_name()?);
    Ok(())
}
