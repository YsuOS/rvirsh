pub mod console;
pub mod delete;
pub mod domain;
pub mod event;
pub mod help;
pub mod hostinfo;
pub mod hostname;
pub mod net;
pub mod pool;
pub mod snapshot;
pub mod template;
pub mod uri;
pub mod version;
pub mod volume;

use anyhow::{anyhow, Context, Result};
use config::Config;
use std::{env, fs::File, io::Read};
use virt::{connect::Connect, domain::Domain};

fn get_conn(settings: &Config) -> Result<Connect> {
    let uri = settings.get_string("URI")?;
    Ok(Connect::open(Some(&uri))?)
}

fn get_xml(cmd: &str) -> Result<String> {
    let xml_path = env::args()
        .nth(2)
        .with_context(|| err_msg("XML file is required", cmd, vec!["<xml path>"]))?;
    let content = xml_to_string(&mut File::open(xml_path)?)?;
    Ok(content)
}

fn get_dom_name(cmd: &str) -> Result<String> {
    let dom_name = env::args()
        .nth(2)
        .with_context(|| err_msg("Domain name is required", cmd, vec!["<domain>"]))?;
    Ok(dom_name)
}

fn get_domain(conn: &Connect, cmd: &str) -> Result<Domain> {
    let dom_name = get_dom_name(cmd)?;

    Ok(Domain::lookup_by_name(conn, &dom_name)?)
}

fn xml_to_string(xml: &mut File) -> Result<String> {
    let mut content = String::new();
    xml.read_to_string(&mut content)?;
    Ok(content)
}

fn bytes_to_gbytes(mem: u64) -> Result<f64> {
    Ok((mem as f64) / 1024.0 / 1024.0 / 1024.0)
}

fn err_msg(msg: &str, cmd: &str, args: Vec<&str>) -> anyhow::Error {
    anyhow!("{}\nUsage: rv {} {}", msg, cmd, args.join(" "))
}

fn get_temp_settings(settings: &Config) -> Result<String> {
    if cfg!(debug_assertions) {
        println!("Use test-templates pool");
        Ok("test-templates".to_string())
    } else {
        settings
            .get_string("TEMP_POOL")
            .with_context(|| anyhow!("Can not get TEMP_POOL from config file"))
    }
}
