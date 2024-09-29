use anyhow::Result;
use virt::storage_pool::StoragePool;

pub fn refresh_pool(pool: &StoragePool) -> Result<()> {
    pool.refresh(0)?;
    Ok(())
}
