use anyhow::{bail, Result};
use virt::storage_pool::StoragePool;

pub fn undefine_pool(pool: &StoragePool) -> Result<()> {
    if pool.is_active()? {
        bail!("Can't undefine active pool")
    }

    pool.undefine()?;
    println!("{} is undefined", pool.get_name()?);
    Ok(())
}
