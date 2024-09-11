use virt::network::Network;

pub fn start_net(net: &Network) {
    if net.is_active().unwrap() {
        println!("{} already started", net.get_name().unwrap());
        return;
    }

    net.create().unwrap();
    println!("{} started", net.get_name().unwrap());
}
