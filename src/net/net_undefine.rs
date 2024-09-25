use anyhow::Result;
use virt::network::Network;

pub fn undefine_net(net: &Network) -> Result<()> {
    net.undefine()?;
    println!("{} is undefined", net.get_name()?);
    Ok(())
}
