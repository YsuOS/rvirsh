use anyhow::Result;
use std::{fs::File, io::Read};
use virt::{connect::Connect, domain::Domain, sys::VIR_DOMAIN_NONE};

pub fn run_domain(conn: &Connect, xml: &mut File) -> Result<()> {
    let mut content = String::new();
    xml.read_to_string(&mut content)?;

    let dom = Domain::create_xml(conn, &content, VIR_DOMAIN_NONE)?;
    println!("Domain {} is running", dom.get_name()?);
    Ok(())
}
