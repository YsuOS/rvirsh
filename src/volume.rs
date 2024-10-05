pub mod vol_create;
pub mod vol_delete;
mod vol_dumpxml;
mod vol_info;
mod vol_key;
mod vol_list;
mod vol_path;
mod vol_pool;
mod vol_wipe;

use crate::{get_conn, get_xml};
use anyhow::{anyhow, bail, Context, Result};
use config::Config;
use std::env;
use virt::{connect::Connect, storage_pool::StoragePool, storage_vol::StorageVol};

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    if cmd == "vol-pool" {
        let volume = get_vol_path(&conn, cmd)?;
        vol_pool::show_volume_pool(&volume)?;
        return Ok(());
    }

    let pool_name = settings.get_string("POOL")?;
    let pool = StoragePool::lookup_by_name(&conn, &pool_name)?;
    crate::pool::pool_refresh::refresh_pool(&pool)?;

    if cmd == "vol-list" {
        vol_list::list_volume(&pool)?;
        return Ok(());
    } else if cmd == "vol-create" {
        let xml = get_xml(cmd)?;

        vol_create::create_vol(&pool, &xml)?;
        return Ok(());
    }

    let volume = get_volume(&pool, cmd)?;

    match cmd {
        "vol-delete" => vol_delete::delete_volume(&volume)?,
        "vol-info" => vol_info::show_volume_info(&volume)?,
        "vol-path" => vol_path::show_volume_path(&volume)?,
        "vol-key" => vol_key::show_volume_key(&volume)?,
        "vol-dumpxml" => vol_dumpxml::show_volume_dumpxml(&volume)?,
        "vol-wipe" => vol_wipe::wipe_volume(&volume)?,
        _ => bail!("{} is not supported", cmd),
    }
    Ok(())
}

fn get_vol_path(conn: &Connect, cmd: &str) -> Result<StorageVol> {
    let vol_path = env::args()
        .nth(2)
        .with_context(|| anyhow!("Volume path is required\nUsage: rv {} <volume path>", cmd))?;
    Ok(StorageVol::lookup_by_path(conn, &vol_path)?)
}

fn get_vol_name(cmd: &str) -> Result<String> {
    let vol_name = env::args()
        .nth(2)
        .with_context(|| anyhow!("Volume name is required\nUsage: rv {} <volume>", cmd))?;
    Ok(vol_name)
}

fn get_volume(pool: &StoragePool, cmd: &str) -> Result<StorageVol> {
    let vol_name = get_vol_name(cmd)?;
    Ok(StorageVol::lookup_by_name(pool, &vol_name)?)
}
