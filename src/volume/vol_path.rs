use anyhow::Result;
use virt::storage_vol::StorageVol;

pub fn show_volume_path(volume: &StorageVol) -> Result<()> {
    println!("{}", volume.get_path()?);
    Ok(())
}
