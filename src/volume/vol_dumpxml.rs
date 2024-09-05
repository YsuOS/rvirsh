use virt::{storage_pool::StoragePool, storage_vol::StorageVol};

pub fn show_volume_dumpxml(pool: &StoragePool, volume: &StorageVol) {
    crate::pool::pool_refresh::refresh_pool(pool);

    println!("{}", volume.get_xml_desc(0).unwrap());
}
