use virt::storage_pool::StoragePool;

pub fn stop_pool(pool: &StoragePool) {
    let _ = pool.destroy();
    println!("{} is stopped", pool.get_name().unwrap());
}
