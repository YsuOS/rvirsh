use anyhow::Result;
use virt::{storage_pool::StoragePool, storage_vol::StorageVol};

pub fn clone_vol(pool: &StoragePool, org_vol: &StorageVol, name: &str) -> Result<()> {
    let xml = org_vol.get_xml_desc(0)?.replace(&org_vol.get_name()?, name);
    let new_vol = StorageVol::create_xml_from(pool, &xml, org_vol, 0)?;
    println!(
        "Volume {} is cloned from {}",
        new_vol.get_name()?,
        org_vol.get_name()?
    );
    Ok(())
}
