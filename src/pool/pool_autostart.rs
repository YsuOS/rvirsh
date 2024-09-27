use anyhow::Result;
use virt::storage_pool::StoragePool;

pub fn autostart_pool(pool: &StoragePool) -> Result<()> {
    pool.set_autostart(true)?;
    println!("{} sets autostart", pool.get_name()?);
    Ok(())
}
