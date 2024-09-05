use virt::storage_pool::StoragePool;

pub fn show_pool_uuid(pool: &StoragePool) {
    println!("{}", pool.get_uuid().unwrap());
}
