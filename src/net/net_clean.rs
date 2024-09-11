use virt::network::Network;

pub fn clean_net(net: &Network) {
    if net.is_active().unwrap() {
        crate::net::net_stop::stop_net(net);
    }

    crate::net::net_undefine::undefine_net(net);
}
