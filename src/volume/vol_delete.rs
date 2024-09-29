use anyhow::Result;
use virt::storage_vol::StorageVol;

pub fn delete_volume(volume: &StorageVol) -> Result<()> {
    volume.delete(0)?;

    println!("Volume {} is deleted", volume.get_name()?);
    Ok(())
}
