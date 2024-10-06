mod template_list;

use anyhow::{anyhow, Context, Result};
use config::Config;
use virt::storage_pool::StoragePool;

use crate::get_conn;

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    let pool_name = settings.get_string("TEMP_POOL")?;
    let pool = StoragePool::lookup_by_name(&conn, &pool_name).with_context(|| {
        anyhow!("No template pool could not be found\nPlease create/define tempalte pool first")
    })?;

    if cmd == "template-list" {
        template_list::list_template(&pool)?;
    }

    Ok(())
}
