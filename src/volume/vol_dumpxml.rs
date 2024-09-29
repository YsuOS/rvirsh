use anyhow::Result;
use virt::storage_vol::StorageVol;

pub fn show_volume_dumpxml(volume: &StorageVol) -> Result<()> {
    println!("{}", volume.get_xml_desc(0)?);
    Ok(())
}
