use virt::network::Network;

pub fn noautostart_net(net: &Network) {
    net.set_autostart(false).unwrap();
    println!("{} unsets autostart", net.get_name().unwrap());
}
