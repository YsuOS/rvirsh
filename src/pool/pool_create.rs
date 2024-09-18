use std::{fs::File, io::Read};
use virt::{connect::Connect, storage_pool::StoragePool, sys::VIR_STORAGE_POOL_CREATE_NORMAL};

pub fn create_pool(conn: &Connect, xml: &mut File) {
    let mut content = String::new();
    xml.read_to_string(&mut content).unwrap();

    let pool = StoragePool::create_xml(conn, &content, VIR_STORAGE_POOL_CREATE_NORMAL).unwrap();
    println!("{} is created", pool.get_name().unwrap());
}
