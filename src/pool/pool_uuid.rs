use anyhow::Result;
use virt::storage_pool::StoragePool;

pub fn show_pool_uuid(pool: &StoragePool) -> Result<()> {
    println!("{}", pool.get_uuid()?);
    Ok(())
}
