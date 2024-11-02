mod template_create;
mod template_list;

use std::env;

use anyhow::{anyhow, Context, Result};
use config::Config;
use virt::storage_pool::StoragePool;

use crate::{err_msg, get_conn, get_temp_settings};

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    let conn = get_conn(settings)?;

    let pool_name = get_temp_settings(settings)?;

    let pool = StoragePool::lookup_by_name(&conn, &pool_name).with_context(|| {
        anyhow!("No template pool could not be found\nPlease create/define tempalte pool first")
    })?;

    if cmd == "template-list" {
        template_list::list_template(&pool)?;
    } else if cmd == "template-create" {
        let template = env::args()
            .nth(2)
            .with_context(|| err_msg("New template name is required", cmd, vec!["<template>"]))?;
        let org_xml = env::args().nth(3).with_context(|| {
            err_msg(
                "New template xml path is required",
                cmd,
                vec!["<template>", "<original xml path>"],
            )
        })?;
        let org_vol = env::args().nth(4).with_context(|| {
            err_msg(
                "New template volume path is required",
                cmd,
                vec![
                    "<template>",
                    "<original xml path>",
                    "<original volume path>",
                ],
            )
        })?;

        template_create::create_template(&pool, &template, &org_xml, &org_vol)?;
    }

    Ok(())
}
