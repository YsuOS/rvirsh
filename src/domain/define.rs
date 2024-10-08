use anyhow::Result;
use std::fs::File;
use virt::{connect::Connect, domain::Domain};

pub fn define_domain(conn: &Connect, xml: &mut File) -> Result<()> {
    let content = crate::xml_to_string(xml)?;

    let dom = Domain::define_xml(conn, &content)?;
    println!("Domain {} is defined", dom.get_name()?);
    Ok(())
}
