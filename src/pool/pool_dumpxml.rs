use virt::storage_pool::StoragePool;

pub fn show_pool_dumpxml(pool: &StoragePool) {
    println!("{}", pool.get_xml_desc(0).unwrap());
}
