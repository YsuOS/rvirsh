use virt::storage_pool::StoragePool;

pub fn autostart_pool(pool: &StoragePool) {
    let _ = pool.set_autostart(true).unwrap();
    println!("{} sets autostart", pool.get_name().unwrap());
}
