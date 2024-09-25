use anyhow::Result;
use virt::connect::Connect;

pub fn list_net(conn: &Connect) -> Result<()> {
    let networks = conn.list_all_networks(0)?;

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
        let name = net.get_name()?;
        let state = if net.is_active()? {
            "active"
        } else {
            "inactive"
        };
        let autostart = crate::net::get_autostart_str(&net)?;
        let persistent = crate::net::get_persistent_str(&net)?;
        println!("{:<10} {:<8} {:<10} {}", name, state, autostart, persistent);
    }
    Ok(())
}
