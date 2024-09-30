mod pool_autostart;
mod pool_clean;
pub mod pool_create;
pub mod pool_define;
mod pool_delete;
mod pool_dumpxml;
mod pool_info;
mod pool_list;
mod pool_noautostart;
pub mod pool_refresh;
mod pool_start;
mod pool_stop;
mod pool_undefine;
mod pool_uuid;

use crate::{get_conn, get_xml};
use anyhow::{anyhow, bail, Context, Result};
use config::Config;
use std::env;
use virt::{connect::Connect, storage_pool::StoragePool};

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    if cmd == "pool-list" {
        pool_list::list_pool(&conn)?;
        return Ok(());
    } else if cmd == "pool-define" || cmd == "pool-create" {
        let xml = get_xml(cmd)?;

        if cmd == "pool-define" {
            pool_define::define_pool(&conn, &xml)?;
        } else if cmd == "pool-create" {
            pool_create::create_pool(&conn, &xml)?;
        }
        return Ok(());
    }

    let pool = get_pool(&conn, cmd)?;

    match cmd {
        "pool-info" => pool_info::show_pool_info(&pool)?,
        "pool-start" => pool_start::start_pool(&pool)?,
        "pool-stop" => pool_stop::stop_pool(&pool)?,
        "pool-refresh" => pool_refresh::refresh_pool(&pool)?,
        "pool-uuid" => pool_uuid::show_pool_uuid(&pool)?,
        "pool-delete" => pool_delete::delete_pool(&pool)?,
        "pool-undefine" => pool_undefine::undefine_pool(&pool)?,
        "pool-clean" => pool_clean::clean_pool(&pool)?,
        "pool-autostart" => pool_autostart::autostart_pool(&pool)?,
        "pool-noautostart" => pool_noautostart::noautostart_pool(&pool)?,
        "pool-dumpxml" => pool_dumpxml::show_pool_dumpxml(&pool)?,
        _ => bail!("{} is not supported", cmd),
    }
    Ok(())
}

fn get_pool_name(cmd: &str) -> Result<String> {
    let pool_name = env::args()
        .nth(2)
        .with_context(|| anyhow!("pool name is required\nUsage: rv {} <pool>", cmd))?;
    Ok(pool_name)
}

fn get_pool(conn: &Connect, cmd: &str) -> Result<StoragePool> {
    let pool_name = get_pool_name(cmd)?;
    Ok(StoragePool::lookup_by_name(conn, &pool_name)?)
}
