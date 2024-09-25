use anyhow::{bail, Result};
use virt::network::Network;

pub fn undefine_net(net: &Network) -> Result<()> {
    if net.is_active()? {
        bail!("Can't undefine active net")
    }

    net.undefine()?;
    println!("{} is undefined", net.get_name()?);
    Ok(())
}
