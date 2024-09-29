use anyhow::Result;
use std::fs::File;
use virt::{connect::Connect, storage_pool::StoragePool};

pub fn define_pool(conn: &Connect, xml: &mut File) -> Result<()> {
    let content = crate::xml_to_string(xml)?;

    let pool = StoragePool::define_xml(conn, &content, 0)?;
    println!("{} is defined", pool.get_name()?);
    Ok(())
}
