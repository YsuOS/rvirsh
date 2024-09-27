use anyhow::Result;
use virt::{
    storage_pool::StoragePool,
    sys::{
        VIR_STORAGE_POOL_BUILDING, VIR_STORAGE_POOL_DEGRADED, VIR_STORAGE_POOL_INACCESSIBLE,
        VIR_STORAGE_POOL_INACTIVE, VIR_STORAGE_POOL_RUNNING,
    },
};

pub fn show_pool_info(pool: &StoragePool) -> Result<()> {
    let poolinfo = pool.get_info()?;
    let state = match poolinfo.state {
        VIR_STORAGE_POOL_INACTIVE => "inactive",
        VIR_STORAGE_POOL_BUILDING => "building",
        VIR_STORAGE_POOL_RUNNING => "running",
        VIR_STORAGE_POOL_DEGRADED => "degraded",
        VIR_STORAGE_POOL_INACCESSIBLE => "inaccessible",
        _ => "-",
    };
    println!("{:<20} {}", "Name:", pool.get_name()?);
    println!("{:<20} {}", "UUID:", pool.get_uuid()?);
    println!("{:<20} {}", "State:", state);
    println!("{:<20} {}", "Persistent:", pool.is_persistent()?);
    println!("{:<20} {}", "Autostart:", pool.get_autostart()?);
    println!(
        "{:<20} {:.2} GiB",
        "Capacity:",
        crate::bytes_to_gbytes(poolinfo.capacity)?
    );
    println!(
        "{:<20} {:.2} GiB",
        "Allocation:",
        crate::bytes_to_gbytes(poolinfo.allocation)?
    );
    println!(
        "{:<20} {:.2} GiB",
        "Available:",
        crate::bytes_to_gbytes(poolinfo.available)?
    );
    Ok(())
}
