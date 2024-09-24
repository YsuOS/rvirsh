use anyhow::Result;
use virt::domain::Domain;

pub fn reset_domain(dom: &Domain) -> Result<()> {
    if !dom.is_active().unwrap() {
        println!("Domain {} is inactive", &dom.get_name().unwrap());
        return Ok(());
    }

    dom.reset().unwrap();
    println!("Domain {} is reset", &dom.get_name().unwrap());
    Ok(())
}
