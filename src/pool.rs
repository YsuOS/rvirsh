mod pool_autostart;
mod pool_clean;
mod pool_create;
mod pool_define;
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

use crate::{
    get_conn,
    help::{help_pool, help_xml},
};
use anyhow::Result;
use config::Config;
use std::{env, fs::File};
use virt::storage_pool::StoragePool;

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    if cmd == "pool-list" {
        pool_list::list_pool(&conn);
        return Ok(());
    } else if cmd == "pool-define" || cmd == "pool-create" {
        let xml_path = env::args().nth(2);
        if xml_path.is_none() {
            help_xml(cmd);
            return Ok(());
        }
        let mut xml = File::open(xml_path.unwrap()).unwrap();
        if cmd == "pool-define" {
            pool_define::define_pool(&conn, &mut xml);
        } else if cmd == "pool-create" {
            pool_create::create_pool(&conn, &mut xml);
        }
        return Ok(());
    }

    let pool_name = env::args().nth(2);
    if pool_name.is_none() {
        help_pool(cmd);
        return Ok(());
    }

    let pool_name = pool_name.unwrap();
    let pool = StoragePool::lookup_by_name(&conn, &pool_name).unwrap();

    match cmd {
        "pool-info" => pool_info::show_pool_info(&pool),
        "pool-start" => pool_start::start_pool(&pool),
        "pool-stop" => pool_stop::stop_pool(&pool),
        "pool-refresh" => pool_refresh::refresh_pool(&pool),
        "pool-uuid" => pool_uuid::show_pool_uuid(&pool),
        "pool-delete" => pool_delete::delete_pool(&pool),
        "pool-undefine" => pool_undefine::undefine_pool(&pool),
        "pool-clean" => pool_clean::clean_pool(&pool),
        "pool-autostart" => pool_autostart::autostart_pool(&pool),
        "pool-noautostart" => pool_noautostart::noautostart_pool(&pool),
        "pool-dumpxml" => pool_dumpxml::show_pool_dumpxml(&pool),
        _ => eprintln!("{} is not supported", cmd),
    }
    Ok(())
}
