use anyhow::{anyhow, Context, Result};
use config::Config;
use virt::storage_pool::StoragePool;

use crate::get_conn;

pub fn main(settings: &Config, _cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    let pool_name = settings.get_string("TEMP_POOL")?;
    let _pool = StoragePool::lookup_by_name(&conn, &pool_name).with_context(|| {
        anyhow!("No template pool could not be found\nPlease create tempalte pool first")
    })?;
    Ok(())
}
