use crate::{get_conn, get_domain, Commands};
use anyhow::{Context, Result};
use config::Config;
use quick_xml::{events::Event, Reader};
use regex::Regex;
use std::path::Path;
use virt::{connect::Connect, domain::Domain, storage_pool::StoragePool, storage_vol::StorageVol};

fn get_volume_from_domain(dom: &Domain) -> Result<StorageVol> {
    let xml = dom.get_xml_desc(0)?;
    let mut reader = Reader::from_str(&xml);
    let mut file_path = None;

    loop {
        match reader.read_event() {
            Ok(Event::Empty(e)) if e.name().as_ref() == b"source" => {
                if let Some(attr) = e
                    .attributes()
                    .find(|a| a.as_ref().unwrap().key.as_ref() == b"file")
                {
                    file_path = Some(attr?.unescape_value()?.to_string());
                }
                break;
            }
            Err(e) => panic!("Error: {}", e),
            Ok(Event::Eof) => break,
            _ => (),
        }
    }
    let file_path = file_path.with_context(|| {
        format!(
            "Could not find volume from {}'s xml",
            dom.get_name().unwrap()
        )
    })?;
    Ok(StorageVol::lookup_by_path(&dom.get_connect()?, &file_path)?)
}

fn get_vol_name(vol_path: &str) -> Result<String> {
    let vol_name = Path::new(vol_path)
        .file_name()
        .and_then(|name| name.to_str())
        .with_context(|| format!("Could not get volume name from volume path"))?;
    Ok(vol_name.to_string())
}

fn create_new_volume_xml(org_vol: &StorageVol, new_vol_path: &str) -> Result<String> {
    let new_vol_xml = org_vol
        .get_xml_desc(0)?
        .replace(&org_vol.get_path()?, new_vol_path)
        .replace(&org_vol.get_name()?, &get_vol_name(new_vol_path)?);
    Ok(new_vol_xml)
}

fn remove_uuid_lines(xml: &str) -> Result<String> {
    let re = Regex::new(r"(?m)^\s*<uuid>.*</uuid>\s*$")?;
    Ok(re.replace_all(xml, "").to_string())
}

fn create_new_domain_xml(
    org_dom: &Domain,
    org_vol: &StorageVol,
    new_name: &str,
    new_vol_path: &str,
) -> Result<String> {
    let org_name = org_dom.get_name()?;
    let new_xml = org_dom
        .get_xml_desc(0)?
        .replace(&org_name, new_name)
        .replace(&org_vol.get_path()?, new_vol_path);
    remove_uuid_lines(&new_xml)
}

/// `rv clone`
pub fn clone_domain(
    conn: &Connect,
    org_dom: &Domain,
    new_name: &str,
    new_pool: &StoragePool,
    new_vol_path: &str,
) -> Result<()> {
    let org_vol = get_volume_from_domain(&org_dom)?;

    let new_vol_xml = create_new_volume_xml(&org_vol, new_vol_path)?;
    let new_xml = create_new_domain_xml(&org_dom, &org_vol, new_name, new_vol_path)?;

    crate::pool::refresh_pool(new_pool)?;
    StorageVol::create_xml_from(new_pool, &new_vol_xml, &org_vol, 0)?;

    let dom = Domain::define_xml(conn, &new_xml)?;
    println!(
        "Domain {} is cloned from {}",
        dom.get_name()?,
        org_dom.get_name()?
    );
    Ok(())
}

pub fn main(settings: &Config, cmd: &Commands) -> Result<()> {
    let conn = get_conn(settings)?;
    let new_pool = StoragePool::lookup_by_name(&conn, &settings.get_string("POOL")?)?;

    if let Commands::Clone(args) = cmd {
        clone_domain(
            &conn,
            &get_domain(&conn, &args.dom)?,
            &args.newdom,
            &new_pool,
            &args.newvol,
        )?;
    }
    return Ok(());
}
