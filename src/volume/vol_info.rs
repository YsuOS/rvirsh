use virt::{
    storage_pool::StoragePool,
    storage_vol::StorageVol,
    sys::{
        VIR_STORAGE_VOL_BLOCK, VIR_STORAGE_VOL_DIR, VIR_STORAGE_VOL_FILE, VIR_STORAGE_VOL_NETDIR,
        VIR_STORAGE_VOL_NETWORK, VIR_STORAGE_VOL_PLOOP,
    },
};

pub fn show_volume_info(pool: &StoragePool, volume: &StorageVol) {
    crate::pool::pool_refresh::refresh_pool(pool);

    let volinfo = volume.get_info().unwrap();
    let kind = match volinfo.kind {
        VIR_STORAGE_VOL_FILE => "file",
        VIR_STORAGE_VOL_BLOCK => "block",
        VIR_STORAGE_VOL_DIR => "dir",
        VIR_STORAGE_VOL_NETWORK => "network",
        VIR_STORAGE_VOL_NETDIR => "netdir",
        VIR_STORAGE_VOL_PLOOP => "ploop",
        _ => "-",
    };

    println!("{:<20} {}", "Name:", volume.get_name().unwrap());
    println!("{:<20} {}", "Type:", kind);
    println!(
        "{:<20} {:.2} GiB",
        "Capacity:",
        (volinfo.capacity as f64) / 1024.0 / 1024.0 / 1024.0
    );
    println!(
        "{:<20} {:.2} GiB",
        "Allocation:",
        (volinfo.allocation as f64) / 1024.0 / 1024.0 / 1024.0
    );
}
