use anyhow::Result;
use virt::domain::Domain;

pub fn shutdown_domain(dom: &Domain) -> Result<()> {
    if !dom.is_active().unwrap() {
        println!("Domain {} is inactive", &dom.get_name().unwrap());
        return Ok(());
    }

    dom.shutdown().unwrap();
    println!("Domain {} shutdowned", &dom.get_name().unwrap());
    Ok(())
}
