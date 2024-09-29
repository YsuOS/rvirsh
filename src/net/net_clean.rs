use anyhow::Result;
use virt::network::Network;

pub fn clean_net(net: &Network) -> Result<()> {
    if net.is_active()? {
        crate::net::net_stop::stop_net(net)?;
    }

    crate::net::net_undefine::undefine_net(net)?;
    Ok(())
}
