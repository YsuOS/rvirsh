use anyhow::Result;
use virt::domain::Domain;

pub fn poweroff_domain(dom: &Domain) -> Result<()> {
    if !dom.is_active().unwrap() {
        println!("Domain {} is inactive", &dom.get_name().unwrap());
        return Ok(());
    }

    dom.destroy().unwrap();
    println!("Domain {} is powered off", &dom.get_name().unwrap());
    Ok(())
}
