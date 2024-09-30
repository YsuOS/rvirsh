use anyhow::Result;
use virt::{connect::Connect, domain::Domain};

pub fn define_domain(conn: &Connect, xml: &str) -> Result<()> {
    let dom = Domain::define_xml(conn, xml)?;
    println!("Domain {} is defined", dom.get_name()?);
    Ok(())
}
