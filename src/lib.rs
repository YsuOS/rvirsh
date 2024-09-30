pub mod delete;
pub mod domain;
pub mod help;
pub mod hostinfo;
pub mod hostname;
pub mod net;
pub mod pool;
pub mod snapshot;
pub mod uri;
pub mod version;
pub mod volume;

use anyhow::{anyhow, Context, Ok, Result};
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
        .with_context(|| anyhow!("XML file is required\nUsage: rv {} <xml path>", cmd))?;
    let content = xml_to_string(&mut File::open(xml_path)?)?;
    Ok(content)
}

fn get_dom_name(cmd: &str) -> Result<String> {
    let dom_name = env::args()
        .nth(2)
        .with_context(|| anyhow!("Domain name is required\nUsage: rv {} <domain>", cmd))?;
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
