use anyhow::Result;
use virt::{storage_pool::StoragePool, storage_vol::StorageVol};

pub fn create_vol(pool: &StoragePool, xml: &str) -> Result<()> {
    let volume = StorageVol::create_xml(pool, xml, 0)?;
    println!("Volume {} is created", volume.get_name()?);
    Ok(())
}
