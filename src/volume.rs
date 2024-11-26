use crate::{get_args, get_conn, get_xml};
use anyhow::{bail, Result};
use config::Config;
use virt::{
    connect::Connect,
    storage_pool::StoragePool,
    storage_vol::StorageVol,
    sys::{
        VIR_STORAGE_VOL_BLOCK, VIR_STORAGE_VOL_DIR, VIR_STORAGE_VOL_FILE, VIR_STORAGE_VOL_NETDIR,
        VIR_STORAGE_VOL_NETWORK, VIR_STORAGE_VOL_PLOOP,
    },
};

fn get_vol_path(conn: &Connect, cmd: &str) -> Result<StorageVol> {
    let vol_path = get_args(2, "Volume path is required", cmd, &vec!["<volume path>"])?;
    Ok(StorageVol::lookup_by_path(conn, &vol_path)?)
}

fn get_volume(pool: &StoragePool, cmd: &str) -> Result<StorageVol> {
    let vol_name = get_args(2, "Volume name is required", cmd, &vec!["<volume>"])?;
    Ok(StorageVol::lookup_by_name(pool, &vol_name)?)
}

/// `rv vol-clone`
pub fn clone_vol(pool: &StoragePool, org_vol: &StorageVol, name: &str) -> Result<StorageVol> {
    let xml = org_vol.get_xml_desc(0)?.replace(&org_vol.get_name()?, name);
    let new_vol = StorageVol::create_xml_from(pool, &xml, org_vol, 0)?;
    println!(
        "Volume {} is cloned from {}",
        new_vol.get_name()?,
        org_vol.get_name()?
    );
    Ok(new_vol)
}

/// `rv vol-create`
pub fn create_vol(pool: &StoragePool, xml: &str) -> Result<()> {
    let volume = StorageVol::create_xml(pool, xml, 0)?;
    println!("Volume {} is created", volume.get_name()?);
    Ok(())
}

/// `rv vol-delete`
pub fn delete_volume(volume: &StorageVol) -> Result<()> {
    volume.delete(0)?;

    println!("Volume {} is deleted", volume.get_name()?);
    Ok(())
}

/// `rv vol-dumpxml`
pub fn show_volume_dumpxml(volume: &StorageVol) -> Result<()> {
    println!("{}", volume.get_xml_desc(0)?);
    Ok(())
}

/// `rv vol-info`
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

/// `rv vol-key`
pub fn show_volume_key(volume: &StorageVol) -> Result<()> {
    println!("{}", volume.get_key()?);
    Ok(())
}

/// `rv vol-list`
pub fn list_volume(pool: &StoragePool) -> Result<()> {
    let vols = pool.list_all_volumes(0)?;

    let width = 25;
    println!("{:<width$} {}", "Name", "Path");
    println!("{:<width$} {}", "-".repeat(25), "-".repeat(35));
    for vol in vols {
        let name = vol.get_name()?;
        let path = vol.get_path()?;
        println!("{:<width$} {}", name, path);
    }
    Ok(())
}

/// `rv vol-path`
pub fn show_volume_path(volume: &StorageVol) -> Result<()> {
    println!("{}", volume.get_path()?);
    Ok(())
}

/// `rv vol-pool`
pub fn show_volume_pool(volume: &StorageVol) -> Result<()> {
    let pool = StoragePool::lookup_by_volume(volume)?;
    println!("{}", pool.get_name()?);
    Ok(())
}

/// `rv vol-wipe`
pub fn wipe_volume(volume: &StorageVol) -> Result<()> {
    volume.wipe(0)?;
    println!("Volume {} is wiped", volume.get_name()?);
    Ok(())
}

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    if cmd == "vol-pool" {
        show_volume_pool(&get_vol_path(&conn, cmd)?)?;
        return Ok(());
    }

    let pool = StoragePool::lookup_by_name(&conn, &settings.get_string("POOL")?)?;
    crate::pool::refresh_pool(&pool)?;

    if cmd == "vol-list" {
        list_volume(&pool)?;
        return Ok(());
    } else if cmd == "vol-create" {
        let xml = get_xml(cmd)?;

        create_vol(&pool, &xml)?;
        return Ok(());
    }

    let volume = get_volume(&pool, cmd)?;

    if cmd == "vol-clone" {
        let name = get_args(
            3,
            "New volume name is required",
            cmd,
            &vec!["<src volume>", "<new volume>"],
        )?;
        clone_vol(&pool, &volume, &name)?;
        return Ok(());
    }

    match cmd {
        "vol-delete" => delete_volume(&volume)?,
        "vol-info" => show_volume_info(&volume)?,
        "vol-path" => show_volume_path(&volume)?,
        "vol-key" => show_volume_key(&volume)?,
        "vol-dumpxml" => show_volume_dumpxml(&volume)?,
        "vol-wipe" => wipe_volume(&volume)?,
        _ => bail!("{} is not supported", cmd),
    }
    Ok(())
}
