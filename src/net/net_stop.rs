use virt::network::Network;

pub fn stop_net(net: &Network) {
    if !net.is_active().unwrap() {
        println!("{} is inactive", &net.get_name().unwrap())
    }

    net.destroy().unwrap();
    println!("{} is stopped", net.get_name().unwrap());
}
