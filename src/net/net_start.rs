use anyhow::{bail, Result};
use virt::network::Network;

pub fn start_net(net: &Network) -> Result<()> {
    if net.is_active()? {
        bail!("{} already started", net.get_name()?)
    }

    net.create()?;
    println!("{} started", net.get_name()?);
    Ok(())
}
