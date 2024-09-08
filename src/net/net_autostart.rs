use virt::network::Network;

pub fn autostart_net(net: &Network) {
    net.set_autostart(true).unwrap();
    println!("{} sets autostart", net.get_name().unwrap());
}
