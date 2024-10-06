use anyhow::Result;
use virt::storage_pool::StoragePool;

pub fn list_template(pool: &StoragePool) -> Result<()> {
    crate::pool::pool_refresh::refresh_pool(pool)?;
    let temps = pool.list_all_volumes(0)?;
    let width = 25;
    println!("{:<width$}", "Name");
    println!("{:<width$}", "-".repeat(25));
    for temp in temps {
        let name = temp.get_name()?;
        println!("{:<width$}", name);
    }
    Ok(())
}
