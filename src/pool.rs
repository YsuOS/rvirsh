mod pool_clean;
mod pool_delete;
mod pool_info;
mod pool_list;
pub mod pool_refresh;
mod pool_stop;
mod pool_undefine;
mod pool_uuid;

use crate::help::help_pool;
use config::Config;
use std::env;
use virt::{connect::Connect, storage_pool::StoragePool};

pub fn main(settings: &Config, cmd: &str) {
    let uri = settings.get_string("URI").unwrap();
    let conn = Connect::open(Some(&uri)).unwrap();

    if cmd == "pool-list" {
        pool_list::list_pool(&conn);
        return;
    }

    let pool_name = env::args().nth(2);
    if pool_name.is_none() {
        help_pool(cmd);
        return;
    }

    let pool_name = pool_name.unwrap();
    let pool = StoragePool::lookup_by_name(&conn, &pool_name).unwrap();

    match cmd {
        "pool-info" => pool_info::show_pool_info(&pool),
        "pool-stop" => pool_stop::stop_pool(&pool),
        "pool-refresh" => pool_refresh::refresh_pool(&pool),
        "pool-uuid" => pool_uuid::show_pool_uuid(&pool),
        "pool-delete" => pool_delete::delete_pool(&pool),
        "pool-undefine" => pool_undefine::undefine_pool(&pool),
        "pool-clean" => pool_clean::clean_pool(&pool),
        _ => eprintln!("{} is not supported", cmd),
    }
}
