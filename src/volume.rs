pub mod vol_delete;
mod vol_dumpxml;
mod vol_info;
mod vol_key;
mod vol_list;
mod vol_path;

use crate::help::help_volume;
use config::Config;
use std::env;
use virt::{connect::Connect, storage_pool::StoragePool, storage_vol::StorageVol};

pub fn main(settings: &Config, cmd: &str) {
    let uri = settings.get_string("URI").unwrap();
    let conn = Connect::open(Some(&uri)).unwrap();
    let pool_name = settings.get_string("POOL").unwrap();
    let pool = StoragePool::lookup_by_name(&conn, &pool_name).unwrap();

    if cmd == "vol-list" {
        vol_list::list_volume(&pool);
        return;
    }

    let vol_name = env::args().nth(2);

    if vol_name.is_none() {
        help_volume(cmd);
        return;
    }

    let vol_name = vol_name.unwrap();
    let volume = StorageVol::lookup_by_name(&pool, &vol_name).unwrap();

    match cmd {
        "vol-delete" => vol_delete::delete_volume(&pool, &volume),
        "vol-info" => vol_info::show_volume_info(&pool, &volume),
        "vol-path" => vol_path::show_volume_path(&pool, &volume),
        "vol-key" => vol_key::show_volume_key(&pool, &volume),
        "vol-dumpxml" => vol_dumpxml::show_volume_dumpxml(&pool, &volume),
        _ => eprintln!("{} is not supported", cmd),
    }
}
