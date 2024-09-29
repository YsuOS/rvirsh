use anyhow::Result;
use std::fs::File;
use virt::{connect::Connect, network::Network};

pub fn define_net(conn: &Connect, xml: &mut File) -> Result<()> {
    let content = crate::xml_to_string(xml)?;

    let net = Network::define_xml(conn, &content)?;
    println!("{} is defined", net.get_name()?);
    Ok(())
}
