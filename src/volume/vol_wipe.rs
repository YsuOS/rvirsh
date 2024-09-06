use virt::{storage_pool::StoragePool, storage_vol::StorageVol};

pub fn wipe_volume(pool: &StoragePool, volume: &StorageVol) {
    crate::pool::pool_refresh::refresh_pool(pool);

    volume.wipe(0).unwrap();

    println!("Volume {} is wiped", volume.get_name().unwrap());
}
