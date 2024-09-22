mod vol_create;
pub mod vol_delete;
mod vol_dumpxml;
mod vol_info;
mod vol_key;
mod vol_list;
mod vol_path;
mod vol_pool;
mod vol_wipe;

use crate::{get_conn, get_xml, help::help_volume};
use anyhow::Result;
use config::Config;
use std::env;
use virt::{storage_pool::StoragePool, storage_vol::StorageVol};

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    if cmd == "vol-pool" {
        let vol_path = env::args().nth(2);
        if vol_path.is_none() {
            crate::help::help_volume_path(cmd);
            return Ok(());
        }
        let vol_path = vol_path.unwrap();

        let volume = StorageVol::lookup_by_path(&conn, &vol_path).unwrap();
        vol_pool::show_volume_pool(&volume);
        return Ok(());
    }

    let pool_name = settings.get_string("POOL").unwrap();
    let pool = StoragePool::lookup_by_name(&conn, &pool_name).unwrap();

    if cmd == "vol-list" {
        vol_list::list_volume(&pool);
        return Ok(());
    } else if cmd == "vol-create" {
        let mut xml = get_xml(cmd)?;

        vol_create::create_vol(&pool, &mut xml);
        return Ok(());
    }

    let vol_name = env::args().nth(2);

    if vol_name.is_none() {
        help_volume(cmd);
        return Ok(());
    }

    let vol_name = vol_name.unwrap();
    let volume = StorageVol::lookup_by_name(&pool, &vol_name).unwrap();

    match cmd {
        "vol-delete" => vol_delete::delete_volume(&pool, &volume),
        "vol-info" => vol_info::show_volume_info(&pool, &volume),
        "vol-path" => vol_path::show_volume_path(&pool, &volume),
        "vol-key" => vol_key::show_volume_key(&pool, &volume),
        "vol-dumpxml" => vol_dumpxml::show_volume_dumpxml(&pool, &volume),
        "vol-wipe" => vol_wipe::wipe_volume(&pool, &volume),
        _ => eprintln!("{} is not supported", cmd),
    }
    Ok(())
}
