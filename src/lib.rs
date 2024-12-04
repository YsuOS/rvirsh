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
use std::{fs::File, io::Read};
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

fn get_domain(conn: &Connect, name: &str) -> Result<Domain> {
    Ok(Domain::lookup_by_name(conn, name)?)
}

fn bytes_to_gbytes(mem: u64) -> Result<f64> {
    Ok((mem as f64) / 1024.0 / 1024.0 / 1024.0)
}

fn get_temp_settings(settings: &Config) -> Result<String> {
    settings
        .get_string("TEMP_POOL")
        .with_context(|| format!("Can not get TEMP_POOL from config file"))
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Commands {
    /// List all domains
    List,
    /// Print domain various information
    Info(Dom),
    /// Define domain
    Define(Xml),
    /// Create and run domain
    Create(Xml),
    /// Clone domain
    Clone(CloneArgs),
    /// Create and run domain from template
    Spawn(TArgs),
    /// Define domain from template
    Deploy(TArgs),
    /// Start domain
    Start(Dom),
    /// Shutdown domain
    Shutdown(Dom),
    /// Reboot domain
    Reboot(Dom),
    /// Suspend domain
    Suspend(Dom),
    /// Resume domain
    Resume(Dom),
    /// Reset domain
    Reset(Dom),
    /// Forcefully terminate domain
    Poweroff(Dom),
    /// Run 'undefine', 'vol-delete', and 'snapshot-delete'
    Delete(Dom),
    /// Connect domain via console
    Console(Dom),
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
    VolCreate(Xml),
    /// Delete volume
    VolDelete(Vol),
    /// Create and run domain snapshot
    SnapshotCreate(SSArgs),
    /// List domain snapshots
    SnapshotList(Dom),
    /// Get current snapshot on the domain
    SnapshotCurrent(Dom),
    /// Set and revert snapshot on the domain
    SnapshotRevert(SSArgs),
    /// Delete a domain snapshots
    SnapshotDelete(Dom),
    /// Define pool
    PoolDefine(Xml),
    /// Create and run pool
    PoolCreate(Xml),
    /// Start pool
    PoolStart(Pool),
    /// List all pools
    PoolList,
    /// Run 'pool-stop', 'pool-delete', and 'pool-undefine'
    PoolClean(Pool),
    /// Create template
    TemplateCreate(TCreateArgs),
    /// List all templates
    TemplateList,
    /// Delete template
    TemplateDelete(Temp),
    /// Print template information
    TemplateInfo(Temp),
    /// Print version
    Version,
    /// Print the hypervisor URI
    Uri,
    /// Print the hypervisor hostname
    Hostname,
    /// Print the host information
    Hostinfo,

    /// Enable autostart
    Autostart(Dom),
    /// Disable autostart
    Noautostart(Dom),
    /// Undefine domain
    Undefine(Dom),
    /// Print domain information. Use `info`
    Dominfo(Dom),
    /// Print domain state. Use `info`
    Domstate(Dom),
    /// Print domain id. Use `info`
    Domid(Dom),
    /// Print domain uuid. Use `info`
    Domuuid(Dom),
    /// Print domain information in XML
    Dumpxml(Dom),
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
    SnapshotInfo(SSArgs),
    /// Get a parent of snapshot
    SnapshotParent(SSArgs),
    /// Print snapshot information in XML
    SnapshotDumpxml(SSArgs),
    /// Print volume information
    VolInfo(Vol),
    /// Print volume key
    VolKey(Vol),
    /// Print volume path
    VolPath(Vol),
    /// Print pool name the volume belongs to
    VolPool(VolPath),
    /// Print volume information in XML
    VolDumpxml(Vol),
    /// Clone volume
    VolClone(VCArgs),
    /// Wipe volume
    VolWipe(Vol),
    /// Print pool information
    PoolInfo(Pool),
    /// Print pool uuid
    PoolUuid(Pool),
    /// Refresh pool
    PoolRefresh(Pool),
    /// Stop pool
    PoolStop(Pool),
    /// Delete pool
    PoolDelete(Pool),
    /// Undefine pool
    PoolUndefine(Pool),
    /// Enable pool autostart
    PoolAutostart(Pool),
    /// Disable pool autostart
    PoolNoautostart(Pool),
    /// Print pool information in XML
    PoolDumpxml(Pool),

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
    /// XML File containing a component's description
    #[arg(value_name = "FILE")]
    name: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct Net {
    /// Network name
    name: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct Dom {
    /// Domain name
    name: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct Vol {
    /// Vol name
    name: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct NewVol {
    /// New vol name
    name: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct VCArgs {
    /// Vol name
    vol: String,
    /// New vol name
    newvol: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct VolPath {
    /// Volume path
    name: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct Pool {
    /// Pool name
    name: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct SSArgs {
    /// Dom name
    dom: String,
    /// Snapshot name
    ss: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct CloneArgs {
    /// Original dom name
    dom: String,
    /// New dom name
    newdom: String,
    /// New vol path
    newvol: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct TCreateArgs {
    /// Temp name
    temp: String,
    /// Original xml name
    orgxml: String,
    /// Original vol path
    orgvol: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct Temp {
    /// Temp name
    name: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct TArgs {
    /// Temp name
    temp: String,
    /// New dom name
    dom: String,
}
