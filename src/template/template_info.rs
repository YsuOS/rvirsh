use crate::template::get_pool_path;
use anyhow::Result;
use virt::storage_pool::StoragePool;

pub fn show_template_info(pool: &StoragePool, template: &str) -> Result<()> {
    let pool_path = get_pool_path(&pool)?;

    println!("{:<10} {}", "Name:", template);
    println!(
        "{:<10} {}",
        "XML:",
        pool_path.clone() + "/" + &template + ".xml"
    );
    println!(
        "{:<10} {}",
        "VOLUME:",
        pool_path.clone() + "/" + &template + ".qcow2"
    );

    Ok(())
}
