use virt::connect::Connect;

pub fn list_net(conn: &Connect) {
    let networks = conn.list_all_networks(0).unwrap();

    println!(
        "{:<10} {:<8} {:<10} {}",
        "Name", "State", "Autostart", "Persistent"
    );
    println!(
        "{:<10} {:<8} {:<10} {}",
        "-".repeat(10),
        "-".repeat(8),
        "-".repeat(10),
        "-".repeat(12)
    );
    for net in networks {
        let name = net.get_name().unwrap();
        let state = if net.is_active().unwrap() {
            "active"
        } else {
            "inactive"
        };
        let autostart = if net.get_autostart().unwrap() {
            "yes"
        } else {
            "no"
        };
        let persistent = if net.is_persistent().unwrap() {
            "yes"
        } else {
            "no"
        };
        println!("{:<10} {:<8} {:<10} {}", name, state, autostart, persistent);
    }
}
