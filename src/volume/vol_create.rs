use std::{fs::File, io::Read};

use virt::{storage_pool::StoragePool, storage_vol::StorageVol};

pub fn create_vol(pool: &StoragePool, xml: &mut File) {
    let mut content = String::new();
    xml.read_to_string(&mut content).unwrap();

    let volume = StorageVol::create_xml(pool, &content, 0).unwrap();
    println!("Volume {} is created", volume.get_name().unwrap());
}
