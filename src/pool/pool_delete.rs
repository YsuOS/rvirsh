use anyhow::{bail, Result};
use virt::storage_pool::StoragePool;

pub fn delete_pool(pool: &StoragePool) -> Result<()> {
    if pool.is_active()? {
        bail!("Can't delete active pool")
    }

    pool.delete(0)?;
    println!("{} is deleted", pool.get_name()?);
    Ok(())
}
