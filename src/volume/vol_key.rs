use anyhow::Result;
use virt::storage_vol::StorageVol;

pub fn show_volume_key(volume: &StorageVol) -> Result<()> {
    println!("{}", volume.get_key()?);
    Ok(())
}
