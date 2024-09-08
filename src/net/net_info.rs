use virt::network::Network;

pub fn show_net_info(net: &Network) {
    let active = if net.is_active().unwrap() {
        "yes"
    } else {
        "no"
    };
    let persistent = crate::net::get_persistent_str(&net);
    let autostart = crate::net::get_autostart_str(&net);

    println!("{:<15} {}", "Name:", net.get_name().unwrap());
    println!("{:<15} {}", "UUID:", net.get_uuid().unwrap());
    println!("{:<15} {}", "Active:", active);
    println!("{:<15} {}", "Persistent:", persistent);
    println!("{:<15} {}", "Autostart:", autostart);
    println!("{:<15} {}", "Bridge:", net.get_bridge_name().unwrap());
}
