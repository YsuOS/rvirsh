use anyhow::{bail, Result};
use virt::{storage_pool::StoragePool, sys::VIR_STORAGE_POOL_CREATE_NORMAL};

pub fn start_pool(pool: &StoragePool) -> Result<()> {
    if !pool.is_active()? {
        bail!("{} is already active", pool.get_name()?)
    }
    pool.create(VIR_STORAGE_POOL_CREATE_NORMAL)?;
    println!("{} is started", pool.get_name()?);
    Ok(())
}
