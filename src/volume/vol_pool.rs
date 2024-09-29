use anyhow::Result;
use virt::{storage_pool::StoragePool, storage_vol::StorageVol};

pub fn show_volume_pool(volume: &StorageVol) -> Result<()> {
    let pool = StoragePool::lookup_by_volume(volume)?;
    println!("{}", pool.get_name()?);
    Ok(())
}
