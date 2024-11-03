use crate::template::get_pool_path;
use anyhow::Result;
use std::{fs, path::Path};
use virt::storage_pool::StoragePool;

pub fn delete_template(pool: &StoragePool, template: &str) -> Result<()> {
    let pool_path = get_pool_path(pool)?;
    fs::remove_file(Path::new(&(pool_path.clone() + "/" + &template + ".xml")))?;
    fs::remove_file(Path::new(&(pool_path.clone() + "/" + &template + ".qcow2")))?;

    println!("Template {} is deleted", template);
    Ok(())
}
