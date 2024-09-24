use anyhow::Result;
use virt::domain::Domain;

pub fn show_domain_state(dom: &Domain) -> Result<()> {
    let state = crate::domain::get_state_str(dom)?;
    println!("{}", state);
    Ok(())
}
