use virt::{storage_pool::StoragePool, storage_vol::StorageVol};

pub fn delete_volume(pool: &StoragePool, volume: &StorageVol) {
    pool.refresh(0).unwrap();
    volume.delete(0).unwrap();

    println!("Volume {} is deleted", volume.get_name().unwrap());
}
