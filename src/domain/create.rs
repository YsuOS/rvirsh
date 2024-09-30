use anyhow::Result;
use virt::{connect::Connect, domain::Domain, sys::VIR_DOMAIN_NONE};

pub fn create_domain(conn: &Connect, xml: &str) -> Result<()> {
    let dom = Domain::create_xml(conn, xml, VIR_DOMAIN_NONE)?;
    println!("Domain {} is created and running", dom.get_name()?);
    Ok(())
}
