use anyhow::Result;
use virt::storage_pool::StoragePool;

pub fn noautostart_pool(pool: &StoragePool) -> Result<()> {
    pool.set_autostart(false)?;
    println!("{} unsets autostart", pool.get_name()?);
    Ok(())
}
