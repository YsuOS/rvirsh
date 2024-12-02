pub mod clone;
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

use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use config::Config;
use std::{env, fs::File, io::Read};
use virt::{connect::Connect, domain::Domain};

fn get_conn(settings: &Config) -> Result<Connect> {
    let uri = settings.get_string("URI")?;
    Ok(Connect::open(Some(&uri))?)
}

/// Get XML file contents
fn get_xml(xml_path: &str) -> Result<String> {
    let mut content = String::new();
    File::open(xml_path)?.read_to_string(&mut content)?;
    Ok(content)
}

fn get_dom_name(cmd: &str) -> Result<String> {
    get_args(2, "Domain name is required", cmd, &vec!["<domain>"])
}

fn get_domain(conn: &Connect, cmd: &str) -> Result<Domain> {
    let dom_name = get_dom_name(cmd)?;
    Ok(Domain::lookup_by_name(conn, &dom_name)?)
}

//fn xml_to_string(xml: &mut File) -> Result<String> {
//    let mut content = String::new();
//    xml.read_to_string(&mut content)?;
//    Ok(content)
//}

fn bytes_to_gbytes(mem: u64) -> Result<f64> {
    Ok((mem as f64) / 1024.0 / 1024.0 / 1024.0)
}

fn get_args(index: usize, msg: &str, cmd: &str, args: &Vec<&str>) -> Result<String> {
    env::args()
        .nth(index)
        .with_context(|| format!("{}\nUsage: rv {} {}", msg, cmd, args.join(" ")))
}

fn get_temp_settings(settings: &Config) -> Result<String> {
    if cfg!(debug_assertions) {
        println!("Use test-templates pool");
        Ok("test-templates".to_string())
    } else {
        settings
            .get_string("TEMP_POOL")
            .with_context(|| format!("Can not get TEMP_POOL from config file"))
    }
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Commands {
    /// List all domains
    List,
    /// Print domain various information
    Info,
    /// Define domain
    Define,
    /// Create and run domain
    Create,
    /// Clone domain
    Clone,
    /// Create and run domain from template
    Spawn,
    /// Define domain from template
    Deploy,
    /// Start domain
    Start,
    /// Shutdown domain
    Shutdown,
    /// Reboot domain
    Reboot,
    /// Suspend domain
    Suspend,
    /// Resume domain
    Resume,
    /// Reset domain
    Reset,
    /// Forcefully terminate domain
    Poweroff,
    /// Run 'undefine', 'vol-delete', and 'snapshot-delete'
    Delete,
    /// Connect domain via console
    Console,
    /// List all networks
    NetList,
    /// Start network
    NetStart(Net),
    /// Create and run network
    NetCreate(Xml),
    /// Define network
    NetDefine(Xml),
    /// Run `net-stop` and `net-undefine`
    NetClean(Net),
    /// List all volumes
    VolList,
    /// Create volume
    VolCreate,
    /// Delete volume
    VolDelete,
    /// Create and run domain snapshot
    SnapshotCreate,
    /// List domain snapshots
    SnapshotList,
    /// Get current snapshot on the domain
    SnapshotCurrent,
    /// Set and revert snapshot on the domain
    SnapshotRevert,
    /// Delete a domain snapshots
    SnapshotDelete,
    /// Define pool
    PoolDefine,
    /// Create and run pool
    PoolCreate,
    /// Start pool
    PoolStart,
    /// List all pools
    PoolList,
    /// Run 'pool-stop', 'pool-delete', and 'pool-undefine'
    PoolClean,
    /// Create template
    TemplateCreate,
    /// List all templates
    TemplateList,
    /// Delete template
    TemplateDelete,
    /// Print template information
    TemplateInfo,
    /// Print version
    Version,
    /// Print the hypervisor URI
    Uri,
    /// Print the hypervisor hostname
    Hostname,
    /// Print the host information
    Hostinfo,

    /// Enable autostart
    Autostart,
    /// Disable autostart
    Noautostart,
    /// Undefine domain
    Undefine,
    /// Print domain information. Use `info`
    Dominfo,
    /// Print domain state. Use `info`
    Domstate,
    /// Print domain id. Use `info`
    Domid,
    /// Print domain uuid. Use `info`
    Domuuid,
    /// Print domain information in XML
    Dumpxml,
    /// Enable net autostart
    NetAutostart(Net),
    /// Disable net autostart
    NetNoautostart(Net),
    /// Stop net
    NetStop(Net),
    /// Undefine net
    NetUndefine(Net),
    /// Print net uuid
    NetUuid(Net),
    /// Print net information
    NetInfo(Net),
    /// Print net information in XML
    NetDumpxml(Net),
    /// Print snapshot information
    SnapshotInfo,
    /// Get a parent of snapshot
    SnapshotParent,
    /// Print snapshot information in XML
    SnapshotDumpxml,
    /// Print volume information
    VolInfo,
    /// Print volume key
    VolKey,
    /// Print volume path
    VolPath,
    /// Print pool name the volume belongs to
    VolPool,
    /// Print volume information in XML
    VolDumpxml,
    /// Clone volume
    VolClone,
    /// Wipe volume
    VolWipe,
    /// Print pool information
    PoolInfo,
    /// Print pool uuid
    PoolUuid,
    /// Refresh pool
    PoolRefresh,
    /// Stop pool
    PoolStop,
    /// Delete pool
    PoolDelete,
    /// Undefine pool
    PoolUndefine,
    /// Enable pool autostart
    PoolAutostart,
    /// Disable pool autostart
    PoolNoautostart,
    /// Print pool information in XML
    PoolDumpxml,

    /// Use 'hostinfo'
    Nodeinfo,
    /// Use 'poweroff'
    Destroy,
    /// Use 'net-stop'
    NetDestroy,
    /// Use 'pool-stop'
    PoolDestroy,
}

#[derive(Args, Debug, PartialEq)]
pub struct Xml {
    /// XML File containing a network description
    #[arg(value_name = "FILE")]
    name: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct Net {
    /// Network name
    name: String,
}
