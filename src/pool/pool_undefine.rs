use virt::storage_pool::StoragePool;

pub fn undefine_pool(pool: &StoragePool) {
    if pool.is_active().unwrap() {
        eprintln!("Can't undefine active pool");
        return;
    }

    let _ = pool.undefine().unwrap();
    println!("{} is undefined", pool.get_name().unwrap());
}
