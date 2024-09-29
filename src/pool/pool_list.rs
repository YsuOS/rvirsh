use anyhow::Result;
use virt::connect::Connect;

pub fn list_pool(conn: &Connect) -> Result<()> {
    let pools = conn.list_all_storage_pools(0)?;

    let n_width = 15;
    let s_width = 10;

    println!("{:<n_width$} {:<s_width$} {}", "Name", "State", "Autostart");
    println!(
        "{:<n_width$} {:<s_width$} {}",
        "-".repeat(15),
        "-".repeat(10),
        "-".repeat(15)
    );
    for pool in pools {
        let name = pool.get_name()?;
        let state = if pool.is_active()? {
            "active"
        } else {
            "inactive"
        };
        let autostart = if pool.get_autostart()? { "yes" } else { "no" };

        println!("{:<n_width$} {:<s_width$} {}", name, state, autostart);
    }
    Ok(())
}
