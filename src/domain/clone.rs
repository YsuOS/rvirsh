use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::path::Path;
use virt::{connect::Connect, domain::Domain, storage_pool::StoragePool, storage_vol::StorageVol};

pub fn clone_domain(
    conn: &Connect,
    org_name: &str,
    org_xml: &str,
    org_vol: &StorageVol,
    new_name: &str,
    new_pool: &StoragePool,
    new_vol_path: &str,
) -> Result<()> {
    let new_vol_xml = create_new_volume_xml(org_vol, new_vol_path)?;
    let new_xml = create_new_domain_xml(org_xml, org_vol, new_vol_path, org_name, new_name)?;

    crate::pool::pool_refresh::refresh_pool(new_pool)?;
    StorageVol::create_xml_from(new_pool, &new_vol_xml, org_vol, 0)?;

    let dom = Domain::define_xml(conn, &new_xml)?;
    println!("Domain {} is cloned from {}", dom.get_name()?, org_name);
    Ok(())
}

fn get_vol_name(vol_path: &str) -> Result<String> {
    let vol_name = Path::new(vol_path)
        .file_name()
        .and_then(|name| name.to_str())
        .context(anyhow!("Could not get volume name from volume path"))?;
    Ok(vol_name.to_string())
}

fn remove_uuid_lines(xml: &str) -> String {
    let re = Regex::new(r"(?m)^\s*<uuid>.*</uuid>\s*$").unwrap();
    re.replace_all(xml, "").to_string()
}

fn create_new_volume_xml(org_vol: &StorageVol, new_vol_path: &str) -> Result<String> {
    let vol_xml = org_vol.get_xml_desc(0)?;
    let updated_xml = vol_xml
        .replace(&org_vol.get_path()?, new_vol_path)
        .replace(&org_vol.get_name()?, &get_vol_name(new_vol_path)?);
    Ok(updated_xml)
}

fn create_new_domain_xml(
    org_xml: &str,
    org_vol: &StorageVol,
    new_vol_path: &str,
    org_name: &str,
    new_name: &str,
) -> Result<String> {
    let new_xml = org_xml
        .replace(org_name, new_name)
        .replace(&org_vol.get_path()?, new_vol_path);
    Ok(remove_uuid_lines(&new_xml))
}
