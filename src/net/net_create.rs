use anyhow::Result;
use std::fs::File;
use virt::{connect::Connect, network::Network};

pub fn create_net(conn: &Connect, xml: &mut File) -> Result<()> {
    let content = crate::xml_to_string(xml)?;

    let net = Network::create_xml(conn, &content)?;
    println!("{} is created", net.get_name()?);
    Ok(())
}
