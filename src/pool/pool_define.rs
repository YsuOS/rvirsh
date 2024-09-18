use std::{fs::File, io::Read};
use virt::{connect::Connect, storage_pool::StoragePool};

pub fn define_pool(conn: &Connect, xml: &mut File) {
    let mut content = String::new();
    xml.read_to_string(&mut content).unwrap();

    let pool = StoragePool::define_xml(conn, &content, 0).unwrap();
    println!("{} is defined", pool.get_name().unwrap());
}
