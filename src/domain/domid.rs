use anyhow::{Context, Result};
use virt::domain::Domain;

pub fn show_domain_id(dom: &Domain) -> Result<()> {
    println!("{}", dom.get_id().context("Can not find ID")?);
    Ok(())
}
