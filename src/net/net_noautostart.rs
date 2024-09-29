use anyhow::Result;
use virt::network::Network;

pub fn noautostart_net(net: &Network) -> Result<()> {
    net.set_autostart(false)?;
    println!("{} unsets autostart", net.get_name()?);
    Ok(())
}
