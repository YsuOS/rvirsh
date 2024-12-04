use crate::{
    get_conn, get_temp_settings, get_xml,
    Commands::{self, *},
};
use anyhow::{Context, Result};
use config::Config;
use quick_xml::{
    events::{attributes::Attribute, BytesStart, BytesText, Event},
    Reader, Writer,
};
use std::{
    fs::{self, File},
    path::Path,
};
use virt::{
    connect::Connect, domain::Domain, storage_pool::StoragePool, storage_vol::StorageVol,
    sys::VIR_DOMAIN_NONE,
};

// TODO: support not only dir type
pub fn get_pool_path(pool: &StoragePool) -> Result<String> {
    let xml = pool.get_xml_desc(0)?;
    let mut reader = Reader::from_str(&xml);
    let mut pool_path: Option<String> = None;
    let mut in_path = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.name().as_ref() == b"path" => {
                in_path = true;
            }
            Ok(Event::Text(e)) if in_path => {
                pool_path = Some(e.unescape()?.to_string());
                break;
            }
            Err(e) => panic!("Error: {}", e),
            Ok(Event::Eof) => break,
            _ => (),
        }
    }
    pool_path.with_context(|| format!("Can not find pool path"))
}

/// Get template components pair; xml data and StorageVol
fn get_template(pool: &StoragePool, temp: &str) -> Result<(String, StorageVol)> {
    // TODO: Be able to get them non-named "temp" + {.xml, .qcow2}
    let (xml_path, _) = get_template_path(pool, temp)?;
    let xml = get_xml(&xml_path)?;
    let vol = StorageVol::lookup_by_name(pool, &get_template_vol_name(&temp)?)?;

    Ok((xml, vol))
}

/// Get vol name
fn get_template_vol_name(name: &str) -> Result<String> {
    Ok(name.to_string() + ".qcow2")
}

/// Get paths of template components pair; xml file and volume file
fn get_template_path(pool: &StoragePool, template: &str) -> Result<(String, String)> {
    let pool_path = get_pool_path(pool)?;
    Ok((
        pool_path.clone() + "/" + template + ".xml",
        pool_path.clone() + "/" + &get_template_vol_name(&template)?,
    ))
}

fn set_name_xml(name: &str, xml: &str) -> String {
    let xml = set_xml(name, xml, "NAME");
    xml.to_string()
}

fn set_fname_xml(name: &str, xml: &str) -> String {
    let xml = set_xml(name, xml, "FILENAME");
    xml.to_string()
}

fn set_xml(name: &str, xml: &str, dst: &str) -> String {
    let xml = xml.replace(dst, name);
    xml.to_string()
}

fn create_sealed_xml(src: &Path, dst: &Path) -> Result<()> {
    let mut reader = Reader::from_file(src)?;
    let mut writer = Writer::new(File::create(dst)?);
    let mut buf = Vec::new();
    let mut in_name = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"name" => {
                writer.write_event(Event::Start(e))?;
                in_name = true;
            }
            Ok(Event::Text(_)) if in_name => {
                writer.write_event(Event::Text(BytesText::new("NAME")))?;
                in_name = false;
            }
            Ok(Event::Empty(e)) if e.name().as_ref() == b"source" => {
                let mut elem = BytesStart::new("source");
                for attr in e.attributes() {
                    if let Ok(a) = attr {
                        if a.key.as_ref() == b"file" {
                            elem.push_attribute(Attribute::from(("file", "FILENAME")));
                        } else {
                            elem.push_attribute(a);
                        }
                    }
                }
                writer.write_event(Event::Empty(elem))?;
            }
            Ok(Event::Eof) => break,
            Ok(e) => writer.write_event(e)?,
            _ => (),
        }
    }
    Ok(())
}

fn create_component_from_tmp(
    pool: &StoragePool,
    name: &str,
    tmp_xml: &mut str,
    tmp_vol: &StorageVol,
) -> Result<(String, StorageVol)> {
    crate::pool::refresh_pool(&pool)?;
    let vol_name = get_template_vol_name(&name)?;
    let vol = crate::volume::clone_vol(pool, tmp_vol, &vol_name)?;
    let xml = set_name_xml(
        &name,
        &set_fname_xml(
            &StorageVol::lookup_by_name(pool, &vol_name)?.get_path()?,
            &tmp_xml,
        ),
    );

    Ok((xml, vol))
}

fn get_temp_pool(conn: &Connect, settings: &Config) -> Result<StoragePool> {
    Ok(
        StoragePool::lookup_by_name(&conn, &get_temp_settings(settings)?).with_context(|| {
            format!("No template pool could not be found\nPlease create/define tempalte pool first")
        })?,
    )
}

/// `rv spawn`
pub fn spawn_domain(
    conn: &Connect,
    pool: &StoragePool,
    name: &str,
    template: &str,
    tmp_xml: &mut str,
    tmp_vol: &StorageVol,
) -> Result<()> {
    let (xml, _) = create_component_from_tmp(pool, name, tmp_xml, tmp_vol)?;
    let dom = Domain::create_xml(conn, &xml, VIR_DOMAIN_NONE)?;

    println!(
        "Domain {} is created from template {}",
        dom.get_name()?,
        template
    );
    Ok(())
}

/// `rv deploy`
pub fn deploy_domain(
    conn: &Connect,
    pool: &StoragePool,
    name: &str,
    template: &str,
    tmp_xml: &mut str,
    tmp_vol: &StorageVol,
) -> Result<()> {
    let (xml, _) = create_component_from_tmp(pool, name, tmp_xml, tmp_vol)?;
    let dom = Domain::define_xml(conn, &xml)?;

    println!(
        "Domain {} is defined from template {}",
        dom.get_name()?,
        template
    );
    Ok(())
}

/// `rv template-delete`
pub fn delete_template(pool: &StoragePool, template: &str) -> Result<()> {
    let (xml_path, vol_path) = get_template_path(pool, template)?;
    fs::remove_file(Path::new(&xml_path))?;
    fs::remove_file(Path::new(&vol_path))?;

    println!("Template {} is deleted", template);
    Ok(())
}

/// `rv template-info`
pub fn show_template_info(pool: &StoragePool, template: &str) -> Result<()> {
    let width = 10;
    let (xml_path, vol_path) = get_template_path(pool, template)?;

    println!("{:<width$} {}", "Name:", template);
    println!("{:<width$} {}", "XML:", &xml_path);
    println!("{:<width$} {}", "VOLUME:", &vol_path);

    Ok(())
}

/// `rv template-list`
pub fn list_template(pool: &StoragePool) -> Result<()> {
    crate::pool::refresh_pool(pool)?;
    let temps = pool.list_all_volumes(0)?;
    let width = 25;
    println!("{:<width$}", "Name");
    println!("{:<width$}", "-".repeat(25));
    for temp in temps {
        println!("{:<width$}", temp.get_name()?);
    }
    Ok(())
}

/// `rv template-create`
pub fn create_template(
    pool: &StoragePool,
    template: &String,
    org_xml: &String,
    org_vol: &String,
) -> Result<()> {
    let (xml_path, vol_path) = get_template_path(pool, template)?;

    create_sealed_xml(Path::new(org_xml), Path::new(&xml_path))?;

    fs::copy(Path::new(org_vol), Path::new(&vol_path))?;

    println!("Template {} is created", template);
    return Ok(());
}

pub fn main(settings: &Config, cmd: &Commands) -> Result<()> {
    let conn = get_conn(settings)?;
    let temp_pool = get_temp_pool(&conn, settings)?;

    match cmd {
        TemplateList => list_template(&temp_pool)?,
        TemplateCreate(args) => {
            create_template(&temp_pool, &args.temp, &args.orgxml, &args.orgvol)?;
        }
        Spawn(args) | Deploy(args) => {
            let (mut xml, vol) = get_template(&temp_pool, &args.temp)?;
            let pool = StoragePool::lookup_by_name(&conn, &settings.get_string("POOL")?)?;

            match cmd {
                Spawn(_) => spawn_domain(&conn, &pool, &args.dom, &args.temp, &mut xml, &vol)?,
                Deploy(_) => deploy_domain(&conn, &pool, &args.dom, &args.temp, &mut xml, &vol)?,
                _ => unreachable!(),
            }
        }
        TemplateDelete(temp) => delete_template(&temp_pool, &temp.name)?,
        TemplateInfo(temp) => show_template_info(&temp_pool, &temp.name)?,
        _ => unreachable!(),
    }
    Ok(())
}
