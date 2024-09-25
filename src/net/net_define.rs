use anyhow::Result;
use std::{fs::File, io::Read};
use virt::{connect::Connect, network::Network};

pub fn define_net(conn: &Connect, xml: &mut File) -> Result<()> {
    let mut content = String::new();
    xml.read_to_string(&mut content)?;

    let net = Network::define_xml(conn, &content)?;
    println!("{} is defined", net.get_name()?);
    Ok(())
}
