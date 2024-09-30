use anyhow::Result;
use virt::{connect::Connect, storage_pool::StoragePool, sys::VIR_STORAGE_POOL_CREATE_NORMAL};

pub fn create_pool(conn: &Connect, xml: &str) -> Result<()> {
    let pool = StoragePool::create_xml(conn, xml, VIR_STORAGE_POOL_CREATE_NORMAL)?;
    println!("{} is created", pool.get_name()?);
    Ok(())
}
