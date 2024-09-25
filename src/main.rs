mod delete;
mod domain;
mod help;
mod hostinfo;
mod hostname;
mod net;
mod pool;
mod snapshot;
mod uri;
mod version;
mod volume;

use anyhow::{anyhow, bail, Context, Ok, Result};
use config::Config;
use std::{env, fs::File, io::Read, path::PathBuf};
use virt::{connect::Connect, domain::Domain};

fn run() -> Result<()> {
    let command = get_command()?;
    let config_file = get_config_file()?;
    let settings = get_settings(&config_file)?;

    match command.as_str() {
        "list" | "start" | "shutdown" | "reboot" | "suspend" | "resume" | "reset" | "poweroff"
        | "undefine" | "dominfo" | "info" | "domid" | "domuuid" | "autostart" | "noautostart"
        | "domstate" | "dumpxml" | "define" | "run" => domain::main(&settings, &command)?,
        "delete" => delete::main(&settings, &command)?,
        "net-list" | "net-uuid" | "net-info" | "net-dumpxml" | "net-autostart"
        | "net-noautostart" | "net-stop" | "net-start" | "net-undefine" | "net-clean"
        | "net-define" | "net-create" => net::main(&settings, &command)?,
        "vol-delete" | "vol-list" | "vol-info" | "vol-path" | "vol-key" | "vol-dumpxml"
        | "vol-pool" | "vol-wipe" | "vol-create" => volume::main(&settings, &command)?,
        "snapshot-list" | "snapshot-delete" | "snapshot-info" | "snapshot-parent"
        | "snapshot-dumpxml" | "snapshot-current" | "snapshot-revert" | "snapshot-create" => {
            snapshot::main(&settings, &command)?
        }
        "pool-list" | "pool-info" | "pool-refresh" | "pool-uuid" | "pool-stop" | "pool-delete"
        | "pool-undefine" | "pool-clean" | "pool-autostart" | "pool-noautostart"
        | "pool-dumpxml" | "pool-start" | "pool-define" | "pool-create" => {
            pool::main(&settings, &command)?
        }
        "version" => version::main(&settings)?,
        "hostname" => hostname::main(&settings)?,
        "hostinfo" => hostinfo::main(&settings)?,
        "uri" => uri::main(&settings)?,
        "nodeinfo" => bail!("'nodeinfo' is deprecated. use 'hostinfo'"),
        "net-destroy" => bail!("'net-destroy' is deprecated. use 'net-stop'"),
        "pool-destroy" => bail!("'pool-destroy' is deprecated. use 'pool-stop'"),
        "destroy" => bail!("'destroy' is deprecated. use 'poweroff'"),
        "help" => help::show_help()?,
        _ => {
            let _ = help::show_help();
            bail!(
                "Command {} is not supported.\n\
                Run 'rv help' to see commands",
                &command
            );
        }
    };

    Ok(())
}

fn get_command() -> Result<String> {
    let command = env::args().nth(1).context(
        "1st argument is required\n\
            Run 'rv help' to see commands",
    )?;
    Ok(command)
}

fn get_config_file() -> Result<PathBuf> {
    if cfg!(debug_assertions) {
        let config_file = env!("CARGO_MANIFEST_DIR").to_string() + "/default.toml";
        println!("Using {}", config_file);
        Ok(PathBuf::from(config_file))
    } else {
        let home_dir = home::home_dir().context("Failed to locate home directory")?;
        let config_file = home_dir.join(".config/rvirsh/default.toml");
        if config_file.exists() {
            Ok(config_file)
        } else {
            bail!("Failed to locate config file; {:?}", config_file)
        }
    }
}

fn get_settings(config_file: &PathBuf) -> Result<Config> {
    let settings = Config::builder()
        .add_source(config::File::with_name(
            config_file.to_str().context("Invalid config file path")?,
        ))
        .build()?;
    Ok(settings)
}

fn main() -> Result<()> {
    run()
}

fn get_conn(settings: &Config) -> Result<Connect> {
    let uri = settings.get_string("URI")?;
    Ok(Connect::open(Some(&uri))?)
}

fn get_xml(cmd: &str) -> Result<File> {
    let xml_path = env::args()
        .nth(2)
        .with_context(|| anyhow!("XML file is required\nUsage: rv {} <xml path>", cmd))?;
    Ok(File::open(xml_path)?)
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
