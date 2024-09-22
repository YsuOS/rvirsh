use anyhow::Result;
use config::Config;
use virt::{storage_pool::StoragePool, storage_vol::StorageVol};

use crate::{get_conn, get_dom_name, get_domain};

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;
    let dom = get_domain(&conn, cmd)?;

    crate::snapshot::snapshot_delete::delete_all_snapshots(&dom);

    crate::domain::undefine::undefine_domain(&dom);

    let pool_name = settings.get_string("POOL").unwrap();
    let pool = StoragePool::lookup_by_name(&conn, &pool_name).unwrap();

    // TODO: Delete only a volume that matches the domain name
    let dom_name = get_dom_name(cmd)?;
    let vol_name = dom_name.clone() + ".qcow2";
    let volume = StorageVol::lookup_by_name(&pool, &vol_name).unwrap();

    crate::volume::vol_delete::delete_volume(&pool, &volume);
    Ok(())
}
