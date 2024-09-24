use anyhow::Result;
use virt::domain::Domain;

pub fn suspend_domain(dom: &Domain) -> Result<()> {
    if !dom.is_active().unwrap() {
        println!("Domain {} is inactive", &dom.get_name().unwrap());
        return Ok(());
    }

    dom.suspend().unwrap();
    println!("Domain {} suspended", &dom.get_name().unwrap());
    Ok(())
}
