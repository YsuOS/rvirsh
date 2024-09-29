use anyhow::Result;
use std::fs::File;
use virt::{connect::Connect, domain::Domain, sys::VIR_DOMAIN_NONE};

pub fn create_domain(conn: &Connect, xml: &mut File) -> Result<()> {
    let content = crate::xml_to_string(xml)?;

    let dom = Domain::create_xml(conn, &content, VIR_DOMAIN_NONE)?;
    println!("Domain {} is created and running", dom.get_name()?);
    Ok(())
}
