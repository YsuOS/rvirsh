pub mod deploy;
pub mod spawn;
mod template_create;
mod template_delete;
mod template_info;
mod template_list;

use anyhow::{anyhow, bail, Context, Result};
use config::Config;
use quick_xml::{events::Event, Reader};
use std::fs::File;
use virt::{storage_pool::StoragePool, storage_vol::StorageVol};

use crate::{get_args, get_conn, get_temp_settings, xml_to_string};

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    let pool_name = get_temp_settings(settings)?;

    let pool = StoragePool::lookup_by_name(&conn, &pool_name).with_context(|| {
        anyhow!("No template pool could not be found\nPlease create/define tempalte pool first")
    })?;

    if cmd == "template-list" {
        template_list::list_template(&pool)?;
        return Ok(());
    }

    let template = get_args(2, "Template name is required", cmd, &vec!["<template>"])?;

    if cmd == "template-create" {
        let usage = vec![
            "<template>",
            "<original xml path>",
            "<original volume path>",
        ];

        let org_xml = get_args(3, "New template xml path is required", cmd, &usage)?;
        let org_vol = get_args(4, "New template volume path is required", cmd, &usage)?;

        template_create::create_template(&pool, &template, &org_xml, &org_vol)?;
        return Ok(());
    } else if cmd == "spawn" || cmd == "deploy" {
        let (vol, mut xml) = get_template(&pool, cmd)?;

        let name = get_args(
            3,
            "New Domain name is required",
            cmd,
            &vec!["<template name>", "<new domain>"],
        )?;

        let pool = StoragePool::lookup_by_name(&conn, &settings.get_string("POOL")?)?;
        if cmd == "spawn" {
            spawn::spawn_domain(&conn, &pool, &name, &template, &mut xml, &vol)?;
        } else {
            deploy::deploy_domain(&conn, &pool, &name, &template, &mut xml, &vol)?;
        }

        return Ok(());
    }

    match cmd {
        "template-delete" => template_delete::delete_template(&pool, &template)?,
        "template-info" => template_info::show_template_info(&pool, &template)?,
        _ => bail!("{} is not supported", cmd),
    }
    Ok(())
}

// only support dir type
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
    pool_path.with_context(|| anyhow!("Can not find pool path"))
}

fn get_template(pool: &StoragePool, cmd: &str) -> Result<(StorageVol, String)> {
    let temp = get_args(2, "Template is required", cmd, &vec!["<template>"])?;
    let vol = StorageVol::lookup_by_name(pool, &(temp.clone() + ".qcow2"))?;
    let xml = xml_to_string(&mut File::open(
        get_pool_path(pool)? + "/" + &(temp.clone() + ".xml"),
    )?)?;

    Ok((vol, xml))
}
