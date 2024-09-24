use anyhow::Result;
use std::{fs::File, io::Read};
use virt::{connect::Connect, domain::Domain};

pub fn define_domain(conn: &Connect, xml: &mut File) -> Result<()> {
    let mut content = String::new();
    xml.read_to_string(&mut content)?;

    let dom = Domain::define_xml(conn, &content)?;
    println!("Domain {} is defined", dom.get_name()?);
    Ok(())
}
