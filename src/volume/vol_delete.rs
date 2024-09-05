use virt::{storage_pool::StoragePool, storage_vol::StorageVol};

pub fn delete_volume(pool: &StoragePool, volume: &StorageVol) {
    crate::pool::pool_refresh::refresh_pool(pool);

    volume.delete(0).unwrap();

    println!("Volume {} is deleted", volume.get_name().unwrap());
}
