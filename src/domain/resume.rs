use anyhow::{bail, Result};
use virt::{domain::Domain, sys::VIR_DOMAIN_PAUSED};

pub fn resume_domain(dom: &Domain) -> Result<()> {
    let (state, _) = dom.get_state()?;
    if state != VIR_DOMAIN_PAUSED {
        bail!("Domain {} is not paused", dom.get_name()?)
    }

    dom.resume()?;
    println!("Domain {} resumed", dom.get_name()?);
    Ok(())
}
