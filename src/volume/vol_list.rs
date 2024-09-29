use anyhow::Result;
use virt::storage_pool::StoragePool;

pub fn list_volume(pool: &StoragePool) -> Result<()> {
    let vols = pool.list_all_volumes(0)?;

    let width = 25;
    println!("{:<width$} {}", "Name", "Path");
    println!("{:<width$} {}", "-".repeat(25), "-".repeat(35));
    for vol in vols {
        let name = vol.get_name()?;
        let path = vol.get_path()?;
        println!("{:<width$} {}", name, path);
    }
    Ok(())
}
