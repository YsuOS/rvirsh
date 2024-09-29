use anyhow::Result;
use virt::network::Network;

pub fn show_net_uuid(net: &Network) -> Result<()> {
    println!("{}", net.get_uuid()?);
    Ok(())
}
