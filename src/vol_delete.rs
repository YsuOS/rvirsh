use config::Config;
use std::env;
use virt::{connect::Connect, storage_pool::StoragePool, storage_vol::StorageVol};

fn show_help() {
    println!("Usage: rvirsh vol-delete <vol>");
}

pub fn delete_volume(conn: &Connect, pool_name: &str, vol_name: &str) {
    let pool = StoragePool::lookup_by_name(conn, pool_name).unwrap();
    pool.refresh(0).unwrap();

    let volume = StorageVol::lookup_by_name(&pool, vol_name).unwrap();
    volume.delete(0).unwrap();

    println!("Volume {} is deleted", vol_name);
}

pub fn main(settings: &Config) {
    let vol_name = env::args().nth(2);

    if vol_name.is_none() {
        eprintln!("Volume name is required");
        show_help();
        return;
    }

    let uri = settings.get_string("URI").unwrap();
    let conn = Connect::open(Some(&uri)).unwrap();

    let vol_name = vol_name.unwrap();
    let pool_name = settings.get_string("POOL").unwrap();

    delete_volume(&conn, &pool_name, &vol_name);
}
