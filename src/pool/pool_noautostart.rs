use virt::storage_pool::StoragePool;

pub fn noautostart_pool(pool: &StoragePool) {
    let _ = pool.set_autostart(false).unwrap();
    println!("{} unsets autostart", pool.get_name().unwrap());
}
