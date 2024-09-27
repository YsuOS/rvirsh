use anyhow::Result;
use std::fs::File;
use virt::{connect::Connect, storage_pool::StoragePool, sys::VIR_STORAGE_POOL_CREATE_NORMAL};

pub fn create_pool(conn: &Connect, xml: &mut File) -> Result<()> {
    let content = crate::xml_to_string(xml)?;

    let pool = StoragePool::create_xml(conn, &content, VIR_STORAGE_POOL_CREATE_NORMAL)?;
    println!("{} is created", pool.get_name()?);
    Ok(())
}
