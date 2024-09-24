use anyhow::Result;
use virt::domain::Domain;

pub fn create_domain(dom: &Domain) -> Result<()> {
    dom.create().unwrap();
    println!("Domain {} started", &dom.get_name().unwrap());
    Ok(())
}
