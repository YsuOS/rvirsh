use virt::storage_pool::StoragePool;

pub fn delete_pool(pool: &StoragePool) {
    if pool.is_active().unwrap() {
        eprintln!("Can't delete active pool");
        return;
    }

    let _ = pool.delete(0).unwrap();
    println!("{} is deleted", pool.get_name().unwrap());
}
