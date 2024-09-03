use virt::{
    storage_pool::StoragePool,
    sys::{
        VIR_STORAGE_POOL_BUILDING, VIR_STORAGE_POOL_DEGRADED, VIR_STORAGE_POOL_INACCESSIBLE,
        VIR_STORAGE_POOL_INACTIVE, VIR_STORAGE_POOL_RUNNING,
    },
};

pub fn show_pool_info(pool: &StoragePool) {
    let poolinfo = pool.get_info().unwrap();
    let state = match poolinfo.state {
        VIR_STORAGE_POOL_INACTIVE => "inactive",
        VIR_STORAGE_POOL_BUILDING => "building",
        VIR_STORAGE_POOL_RUNNING => "running",
        VIR_STORAGE_POOL_DEGRADED => "degraded",
        VIR_STORAGE_POOL_INACCESSIBLE => "inaccessible",
        _ => "-",
    };
    println!("{:<20} {}", "Name:", pool.get_name().unwrap());
    println!("{:<20} {}", "UUID:", pool.get_uuid().unwrap());
    println!("{:<20} {}", "State:", state);
    println!("{:<20} {}", "Persistent:", pool.is_persistent().unwrap());
    println!("{:<20} {}", "Autostart:", pool.get_autostart().unwrap());
    println!(
        "{:<20} {:.2} GiB",
        "Capacity:",
        (poolinfo.capacity as f64) / 1024.0 / 1024.0 / 1024.0
    );
    println!(
        "{:<20} {:.2} GiB",
        "Allocation:",
        (poolinfo.allocation as f64) / 1024.0 / 1024.0 / 1024.0
    );
    println!(
        "{:<20} {:.2} GiB",
        "Available:",
        (poolinfo.available as f64) / 1024.0 / 1024.0 / 1024.0
    );
}
