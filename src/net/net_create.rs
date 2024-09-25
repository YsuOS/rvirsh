use anyhow::Result;
use std::{fs::File, io::Read};
use virt::{connect::Connect, network::Network};

pub fn create_net(conn: &Connect, xml: &mut File) -> Result<()> {
    let mut content = String::new();
    xml.read_to_string(&mut content)?;

    let net = Network::create_xml(conn, &content)?;
    println!("{} is created", net.get_name()?);
    Ok(())
}
