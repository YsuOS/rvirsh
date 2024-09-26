use std::fs::File;

use anyhow::Result;
use virt::{storage_pool::StoragePool, storage_vol::StorageVol};

pub fn create_vol(pool: &StoragePool, xml: &mut File) -> Result<()> {
    let content = crate::xml_to_string(xml)?;

    let volume = StorageVol::create_xml(pool, &content, 0)?;
    println!("Volume {} is created", volume.get_name()?);
    Ok(())
}
