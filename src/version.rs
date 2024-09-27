use anyhow::Result;
use config::Config;

use crate::get_conn;

fn calc_version(version: u32) -> String {
    format!(
        "{}.{}.{}",
        version / 1000000,
        (version % 1000000) / 1000,
        version % 1000
    )
}

pub fn main(settings: &Config) -> Result<()> {
    let conn = get_conn(settings)?;

    let lib_ver = calc_version(conn.get_lib_version()?);
    let hyp_ver = calc_version(conn.get_hyp_version()?);

    //println!("Compiled againt library: libvirt {}", lib_ver);
    println!("Using Library: libvirt {}", lib_ver);
    //println!("Using API: QEMU {}", lib_ver);
    println!("Running hypervisor: QEMU {}", hyp_ver);
    Ok(())
}
