use anyhow::Result;
use virt::domain::Domain;

pub fn undefine_domain(dom: &Domain) -> Result<()> {
    if dom.is_active().unwrap() {
        dom.destroy().unwrap();
    }

    dom.undefine().unwrap();
    println!("Domain {} is undefined", dom.get_name().unwrap());
    Ok(())
}
