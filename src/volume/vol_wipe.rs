use anyhow::Result;
use virt::storage_vol::StorageVol;

pub fn wipe_volume(volume: &StorageVol) -> Result<()> {
    volume.wipe(0)?;
    println!("Volume {} is wiped", volume.get_name()?);
    Ok(())
}
