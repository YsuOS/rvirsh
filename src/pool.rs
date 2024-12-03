use crate::{
    get_conn, get_xml,
    Commands::{self, *},
};
use anyhow::{bail, Result};
use config::Config;
use virt::{
    connect::Connect,
    storage_pool::StoragePool,
    sys::{
        VIR_STORAGE_POOL_BUILDING, VIR_STORAGE_POOL_CREATE_NORMAL, VIR_STORAGE_POOL_DEGRADED,
        VIR_STORAGE_POOL_INACCESSIBLE, VIR_STORAGE_POOL_INACTIVE, VIR_STORAGE_POOL_RUNNING,
    },
};

/// `rv pool-autostart`
pub fn autostart_pool(pool: &StoragePool) -> Result<()> {
    pool.set_autostart(true)?;
    println!("{} sets autostart", pool.get_name()?);
    Ok(())
}

/// `rv pool-clean`
pub fn clean_pool(pool: &StoragePool) -> Result<()> {
    if pool.is_active()? {
        stop_pool(pool)?;
    }
    delete_pool(pool)?;
    undefine_pool(pool)?;
    Ok(())
}

/// `rv pool-create`
pub fn create_pool(conn: &Connect, xml: &str) -> Result<()> {
    let pool = StoragePool::create_xml(conn, xml, VIR_STORAGE_POOL_CREATE_NORMAL)?;
    println!("{} is created", pool.get_name()?);
    Ok(())
}

/// `rv pool-define`
pub fn define_pool(conn: &Connect, xml: &str) -> Result<()> {
    let pool = StoragePool::define_xml(conn, xml, 0)?;
    println!("{} is defined", pool.get_name()?);
    Ok(())
}

/// `rv pool-delete`
pub fn delete_pool(pool: &StoragePool) -> Result<()> {
    if pool.is_active()? {
        bail!("Can't delete active pool")
    }

    pool.delete(0)?;
    println!("{} is deleted", pool.get_name()?);
    Ok(())
}

/// `rv pool-dumpxml`
pub fn show_pool_dumpxml(pool: &StoragePool) -> Result<()> {
    println!("{}", pool.get_xml_desc(0)?);
    Ok(())
}

/// `rv pool-info`
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

/// `rv pool-list`
pub fn list_pool(conn: &Connect) -> Result<()> {
    let pools = conn.list_all_storage_pools(0)?;

    let n_width = 15;
    let s_width = 10;

    println!("{:<n_width$} {:<s_width$} {}", "Name", "State", "Autostart");
    println!(
        "{:<n_width$} {:<s_width$} {}",
        "-".repeat(15),
        "-".repeat(10),
        "-".repeat(15)
    );
    for pool in pools {
        let name = pool.get_name()?;
        let state = if pool.is_active()? {
            "active"
        } else {
            "inactive"
        };
        let autostart = if pool.get_autostart()? { "yes" } else { "no" };

        println!("{:<n_width$} {:<s_width$} {}", name, state, autostart);
    }
    Ok(())
}

/// `rv pool-noautostart`
pub fn noautostart_pool(pool: &StoragePool) -> Result<()> {
    pool.set_autostart(false)?;
    println!("{} unsets autostart", pool.get_name()?);
    Ok(())
}

/// `rv pool-refresh`
pub fn refresh_pool(pool: &StoragePool) -> Result<()> {
    pool.refresh(0)?;
    Ok(())
}

/// `rv pool-start`
pub fn start_pool(pool: &StoragePool) -> Result<()> {
    if pool.is_active()? {
        bail!("{} is already active", pool.get_name()?)
    }
    pool.create(VIR_STORAGE_POOL_CREATE_NORMAL)?;
    println!("{} is started", pool.get_name()?);
    Ok(())
}

/// `rv pool-stop`
pub fn stop_pool(pool: &StoragePool) -> Result<()> {
    pool.destroy()?;
    println!("{} is stopped", pool.get_name()?);
    Ok(())
}

/// `rv pool-undefine`
pub fn undefine_pool(pool: &StoragePool) -> Result<()> {
    if pool.is_active()? {
        bail!("Can't undefine active pool")
    }

    pool.undefine()?;
    println!("{} is undefined", pool.get_name()?);
    Ok(())
}

/// `rv pool-uuid`
pub fn show_pool_uuid(pool: &StoragePool) -> Result<()> {
    println!("{}", pool.get_uuid()?);
    Ok(())
}

pub fn main(settings: &Config, cmd: &Commands) -> Result<()> {
    let conn = get_conn(settings)?;

    match cmd {
        PoolList => {
            list_pool(&conn)?;
        }
        PoolDefine(xml) | PoolCreate(xml) => {
            let xml = get_xml(&xml.name)?;

            match cmd {
                PoolDefine(_) => define_pool(&conn, &xml)?,
                PoolCreate(_) => create_pool(&conn, &xml)?,
                _ => unreachable!(),
            }
        }
        PoolInfo(pool)
        | PoolRefresh(pool)
        | PoolUuid(pool)
        | PoolStop(pool)
        | PoolDelete(pool)
        | PoolUndefine(pool)
        | PoolClean(pool)
        | PoolAutostart(pool)
        | PoolNoautostart(pool)
        | PoolDumpxml(pool)
        | PoolStart(pool) => {
            let pool = StoragePool::lookup_by_name(&conn, &pool.name)?;

            match cmd {
                PoolInfo(_) => show_pool_info(&pool)?,
                PoolStart(_) => start_pool(&pool)?,
                PoolStop(_) => stop_pool(&pool)?,
                PoolRefresh(_) => refresh_pool(&pool)?,
                PoolUuid(_) => show_pool_uuid(&pool)?,
                PoolDelete(_) => delete_pool(&pool)?,
                PoolUndefine(_) => undefine_pool(&pool)?,
                PoolClean(_) => clean_pool(&pool)?,
                PoolAutostart(_) => autostart_pool(&pool)?,
                PoolNoautostart(_) => noautostart_pool(&pool)?,
                PoolDumpxml(_) => show_pool_dumpxml(&pool)?,
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}
