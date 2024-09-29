use anyhow::{bail, Result};
use virt::network::Network;

pub fn stop_net(net: &Network) -> Result<()> {
    if !net.is_active()? {
        bail!("{} is inactive", net.get_name()?)
    }

    net.destroy()?;
    println!("{} is stopped", net.get_name()?);
    Ok(())
}
