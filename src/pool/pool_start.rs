use virt::{storage_pool::StoragePool, sys::VIR_STORAGE_POOL_CREATE_NORMAL};

pub fn start_pool(pool: &StoragePool) {
    if !pool.is_active().unwrap() {
        eprintln!("{} is already active", pool.get_name().unwrap());
        return;
    }
    let _ = pool.create(VIR_STORAGE_POOL_CREATE_NORMAL);
    println!("{} is started", pool.get_name().unwrap());
}
