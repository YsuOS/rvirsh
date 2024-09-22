use anyhow::Result;
use config::Config;
use std::env;
use virt::{domain::Domain, storage_pool::StoragePool, storage_vol::StorageVol};

use crate::{get_conn, help::help_domain};

pub fn main(settings: &Config) -> Result<()> {
    let dom_name = env::args().nth(2);

    if dom_name.is_none() {
        help_domain("delete");
        return Ok(());
    }

    let conn = get_conn(settings)?;

    let dom_name = dom_name.unwrap();
    let dom = Domain::lookup_by_name(&conn, &dom_name).unwrap();

    crate::snapshot::snapshot_delete::delete_all_snapshots(&dom);

    crate::domain::undefine::undefine_domain(&dom);

    let pool_name = settings.get_string("POOL").unwrap();
    let pool = StoragePool::lookup_by_name(&conn, &pool_name).unwrap();

    // TODO: Delete only a volume that matches the domain name
    let vol_name = dom_name.clone() + ".qcow2";
    let volume = StorageVol::lookup_by_name(&pool, &vol_name).unwrap();

    crate::volume::vol_delete::delete_volume(&pool, &volume);
    Ok(())
}
