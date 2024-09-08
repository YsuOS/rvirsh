use virt::network::Network;

pub fn show_net_uuid(net: &Network) {
    println!("{}", net.get_uuid().unwrap());
}
