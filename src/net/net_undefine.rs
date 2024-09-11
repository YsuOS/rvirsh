use virt::network::Network;

pub fn undefine_net(net: &Network) {
    if net.is_active().unwrap() {
        eprintln!("Can't undefine active net");
        return;
    }

    net.undefine().unwrap();
    println!("{} is undefined", net.get_name().unwrap());
}
