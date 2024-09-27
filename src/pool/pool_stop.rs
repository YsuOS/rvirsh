use anyhow::Result;
use virt::storage_pool::StoragePool;

pub fn stop_pool(pool: &StoragePool) -> Result<()> {
    pool.destroy()?;
    println!("{} is stopped", pool.get_name()?);
    Ok(())
}
