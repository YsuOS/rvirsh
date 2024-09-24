use anyhow::Result;
use virt::domain::Domain;

pub fn reboot_domain(dom: &Domain) -> Result<()> {
    if !dom.is_active()? {
        crate::domain::error_domain_inactive(dom)?
    }
    dom.reboot(0)?;
    println!("Domain {} rebooted", dom.get_name()?);
    Ok(())
}
