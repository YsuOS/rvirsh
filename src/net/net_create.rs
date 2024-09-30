use anyhow::Result;
use virt::{connect::Connect, network::Network};

pub fn create_net(conn: &Connect, xml: &str) -> Result<()> {
    let net = Network::create_xml(conn, xml)?;
    println!("{} is created", net.get_name()?);
    Ok(())
}
