use crate::{get_args, get_conn, get_temp_settings, get_xml};
use anyhow::{bail, Context, Result};
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

fn tmp_create_get_args(index: usize, msg: &str, cmd: &str) -> Result<String> {
    let usage = vec![
        "<template>",
        "<original xml path>",
        "<original volume path>",
    ];
    get_args(index, msg, cmd, &usage)
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

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    let temp_pool = StoragePool::lookup_by_name(&conn, &get_temp_settings(settings)?)
        .with_context(|| {
            format!("No template pool could not be found\nPlease create/define tempalte pool first")
        })?;

    if cmd == "template-list" {
        list_template(&temp_pool)?;
        return Ok(());
    }

    let template = get_args(
        2,
        "Template name is required",
        cmd,
        &vec!["<template name>"],
    )?;

    if cmd == "template-create" {
        let org_xml = tmp_create_get_args(3, "Original template xml path is required", cmd)?;
        let org_vol = tmp_create_get_args(4, "Original template volume path is required", cmd)?;

        create_template(&temp_pool, &template, &org_xml, &org_vol)?;
        return Ok(());
    } else if cmd == "spawn" || cmd == "deploy" {
        let name = get_args(
            3,
            "New Domain name is required",
            cmd,
            &vec!["<template name>", "<new domain>"],
        )?;

        let (mut xml, vol) = get_template(&temp_pool, &template)?;
        let pool = StoragePool::lookup_by_name(&conn, &settings.get_string("POOL")?)?;

        if cmd == "spawn" {
            spawn_domain(&conn, &pool, &name, &template, &mut xml, &vol)?;
        } else if cmd == "deploy" {
            deploy_domain(&conn, &pool, &name, &template, &mut xml, &vol)?;
        }

        return Ok(());
    }

    match cmd {
        "template-delete" => delete_template(&temp_pool, &template)?,
        "template-info" => show_template_info(&temp_pool, &template)?,
        _ => bail!("{} is not supported", cmd),
    }
    Ok(())
}
