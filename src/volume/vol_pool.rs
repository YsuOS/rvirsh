use virt::{storage_pool::StoragePool, storage_vol::StorageVol};

pub fn show_volume_pool(volume: &StorageVol) {
    let pool = StoragePool::lookup_by_volume(volume).unwrap();
    println!("{}", pool.get_name().unwrap());
}
