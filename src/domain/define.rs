use anyhow::Result;
use std::fs::File;
use virt::{connect::Connect, domain::Domain};

use crate::domain::xml_to_string;

pub fn define_domain(conn: &Connect, xml: &mut File) -> Result<()> {
    let content = xml_to_string(xml)?;

    let dom = Domain::define_xml(conn, &content)?;
    println!("Domain {} is defined", dom.get_name()?);
    Ok(())
}
