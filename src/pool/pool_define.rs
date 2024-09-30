use anyhow::Result;
use virt::{connect::Connect, storage_pool::StoragePool};

pub fn define_pool(conn: &Connect, xml: &str) -> Result<()> {
    let pool = StoragePool::define_xml(conn, xml, 0)?;
    println!("{} is defined", pool.get_name()?);
    Ok(())
}
