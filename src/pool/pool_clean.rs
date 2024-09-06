use virt::storage_pool::StoragePool;

pub fn clean_pool(pool: &StoragePool) {
    if pool.is_active().unwrap() {
        crate::pool::pool_stop::stop_pool(pool);
    }
    crate::pool::pool_delete::delete_pool(pool);
    crate::pool::pool_undefine::undefine_pool(pool);
}
