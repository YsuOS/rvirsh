use anyhow::Result;
use virt::{
    storage_vol::StorageVol,
    sys::{
        VIR_STORAGE_VOL_BLOCK, VIR_STORAGE_VOL_DIR, VIR_STORAGE_VOL_FILE, VIR_STORAGE_VOL_NETDIR,
        VIR_STORAGE_VOL_NETWORK, VIR_STORAGE_VOL_PLOOP,
    },
};

pub fn show_volume_info(volume: &StorageVol) -> Result<()> {
    let volinfo = volume.get_info()?;
    let kind = match volinfo.kind {
        VIR_STORAGE_VOL_FILE => "file",
        VIR_STORAGE_VOL_BLOCK => "block",
        VIR_STORAGE_VOL_DIR => "dir",
        VIR_STORAGE_VOL_NETWORK => "network",
        VIR_STORAGE_VOL_NETDIR => "netdir",
        VIR_STORAGE_VOL_PLOOP => "ploop",
        _ => "-",
    };

    println!("{:<20} {}", "Name:", volume.get_name()?);
    println!("{:<20} {}", "Type:", kind);
    println!(
        "{:<20} {:.2} GiB",
        "Capacity:",
        crate::bytes_to_gbytes(volinfo.capacity)?
    );
    println!(
        "{:<20} {:.2} GiB",
        "Allocation:",
        crate::bytes_to_gbytes(volinfo.allocation)?
    );
    Ok(())
}
