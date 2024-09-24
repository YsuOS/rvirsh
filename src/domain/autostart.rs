use anyhow::Result;
use virt::domain::Domain;

pub fn autostart_domain(dom: &Domain) -> Result<()> {
    dom.set_autostart(true)?;
    println!("{} sets autostart", dom.get_name()?);
    Ok(())
}
