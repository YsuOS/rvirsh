use anyhow::Result;
use virt::network::Network;

pub fn autostart_net(net: &Network) -> Result<()> {
    net.set_autostart(true)?;
    println!("{} sets autostart", net.get_name()?);
    Ok(())
}
