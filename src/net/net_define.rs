use anyhow::Result;
use virt::{connect::Connect, network::Network};

pub fn define_net(conn: &Connect, xml: &str) -> Result<()> {
    let net = Network::define_xml(conn, xml)?;
    println!("{} is defined", net.get_name()?);
    Ok(())
}
