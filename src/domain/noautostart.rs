use anyhow::Result;
use virt::domain::Domain;

pub fn noautostart_domain(dom: &Domain) -> Result<()> {
    dom.set_autostart(false).unwrap();
    println!("{} unsets autostart", dom.get_name().unwrap());
    Ok(())
}
