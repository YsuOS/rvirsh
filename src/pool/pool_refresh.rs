use virt::storage_pool::StoragePool;

pub fn refresh_pool(pool: &StoragePool) {
    let _ = pool.refresh(0);
    println!("{} is refreshed", pool.get_name().unwrap());
}
