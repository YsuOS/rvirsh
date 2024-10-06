use anyhow::{bail, Context, Result};
use config::Config;
use std::{env, path::PathBuf};

fn run() -> Result<()> {
    let command = get_command()?;
    let config_file = get_config_file()?;
    let settings = get_settings(&config_file)?;

    match command.as_str() {
        "list" | "start" | "shutdown" | "reboot" | "suspend" | "resume" | "reset" | "poweroff"
        | "undefine" | "dominfo" | "info" | "domid" | "domuuid" | "autostart" | "noautostart"
        | "domstate" | "dumpxml" | "define" | "create" => {
            rvirsh::domain::main(&settings, &command)?
        }
        "delete" => rvirsh::delete::main(&settings, &command)?,
        "net-list" | "net-uuid" | "net-info" | "net-dumpxml" | "net-autostart"
        | "net-noautostart" | "net-stop" | "net-start" | "net-undefine" | "net-clean"
        | "net-define" | "net-create" => rvirsh::net::main(&settings, &command)?,
        "vol-delete" | "vol-list" | "vol-info" | "vol-path" | "vol-key" | "vol-dumpxml"
        | "vol-pool" | "vol-wipe" | "vol-create" => rvirsh::volume::main(&settings, &command)?,
        "snapshot-list" | "snapshot-delete" | "snapshot-info" | "snapshot-parent"
        | "snapshot-dumpxml" | "snapshot-current" | "snapshot-revert" | "snapshot-create" => {
            rvirsh::snapshot::main(&settings, &command)?
        }
        "pool-list" | "pool-info" | "pool-refresh" | "pool-uuid" | "pool-stop" | "pool-delete"
        | "pool-undefine" | "pool-clean" | "pool-autostart" | "pool-noautostart"
        | "pool-dumpxml" | "pool-start" | "pool-define" | "pool-create" => {
            rvirsh::pool::main(&settings, &command)?
        }
        "template-list" | "template-create" | "template-delete" | "template-info" => {
            rvirsh::template::main(&settings, &command)?
        }
        "version" => rvirsh::version::main(&settings)?,
        "hostname" => rvirsh::hostname::main(&settings)?,
        "hostinfo" => rvirsh::hostinfo::main(&settings)?,
        "uri" => rvirsh::uri::main(&settings)?,
        "nodeinfo" => bail!("'nodeinfo' is deprecated. use 'hostinfo'"),
        "net-destroy" => bail!("'net-destroy' is deprecated. use 'net-stop'"),
        "pool-destroy" => bail!("'pool-destroy' is deprecated. use 'pool-stop'"),
        "destroy" => bail!("'destroy' is deprecated. use 'poweroff'"),
        "help" => rvirsh::help::show_help()?,
        _ => {
            let _ = rvirsh::help::show_help();
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
