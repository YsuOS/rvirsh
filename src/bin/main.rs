use anyhow::{bail, Context, Result};
use clap::Parser;
use config::Config;
use rvirsh::Commands::{self, *};
use std::{env, path::PathBuf};

#[derive(Parser)]
#[command(about = None, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    let config_file = get_config_file(&cli.config)?;
    let settings = get_settings(&config_file)?;

    let command = cli.command;
    match command {
        List | Start | Shutdown | Reboot | Suspend | Resume | Reset | Poweroff | Undefine
        | Dominfo | Info | Domid | Domuuid | Autostart | Noautostart | Domstate | Dumpxml
        | Define | Create => todo!(),
        Clone => todo!(),
        Delete => todo!(),
        Console => todo!(),
        NetList | NetUuid(_) | NetInfo(_) | NetDumpxml(_) | NetAutostart(_) | NetNoautostart(_)
        | NetStop(_) | NetStart(_) | NetUndefine(_) | NetClean(_) | NetDefine(_) | NetCreate(_) => {
            rvirsh::net::main(&settings, &command)?
        }
        VolDelete | VolList | VolInfo | VolPath | VolKey | VolDumpxml | VolPool | VolWipe
        | VolCreate | VolClone => todo!(),
        SnapshotList | SnapshotDelete | SnapshotInfo | SnapshotParent | SnapshotDumpxml
        | SnapshotCurrent | SnapshotRevert | SnapshotCreate => todo!(),
        PoolList | PoolInfo | PoolRefresh | PoolUuid | PoolStop | PoolDelete | PoolUndefine
        | PoolClean | PoolAutostart | PoolNoautostart | PoolDumpxml | PoolStart | PoolDefine
        | PoolCreate => todo!(),
        TemplateList | TemplateCreate | TemplateDelete | TemplateInfo | Spawn | Deploy => todo!(),
        Version => rvirsh::version::main(&settings)?,
        Uri => rvirsh::uri::main(&settings)?,
        Hostname => rvirsh::hostname::main(&settings)?,
        Hostinfo => rvirsh::hostinfo::main(&settings)?,
        Nodeinfo => deprecated_err(&command, "hostinfo")?,
        Destroy => deprecated_err(&command, "poweroff")?,
        NetDestroy => deprecated_err(&command, "net-stop")?,
        PoolDestroy => deprecated_err(&command, "pool-stop")?,
    };

    //    match command.as_str() {
    //        "list" | "start" | "shutdown" | "reboot" | "suspend" | "resume" | "reset" | "poweroff"
    //        | "undefine" | "dominfo" | "info" | "domid" | "domuuid" | "autostart" | "noautostart"
    //        | "domstate" | "dumpxml" | "define" | "create" => {
    //            rvirsh::domain::main(&settings, &command)?
    //        }
    //        "clone" => rvirsh::clone::main(&settings, &command)?,
    //        "delete" => rvirsh::delete::main(&settings, &command)?,
    //        "console" => rvirsh::console::main(&settings, &command)?,
    //        "net-list" | "net-uuid" | "net-info" | "net-dumpxml" | "net-autostart"
    //        | "net-noautostart" | "net-stop" | "net-start" | "net-undefine" | "net-clean"
    //        | "net-define" | "net-create" => rvirsh::net::main(&settings, &command)?,
    //        "vol-delete" | "vol-list" | "vol-info" | "vol-path" | "vol-key" | "vol-dumpxml"
    //        | "vol-pool" | "vol-wipe" | "vol-create" | "vol-clone" => {
    //            rvirsh::volume::main(&settings, &command)?
    //        }
    //        "snapshot-list" | "snapshot-delete" | "snapshot-info" | "snapshot-parent"
    //        | "snapshot-dumpxml" | "snapshot-current" | "snapshot-revert" | "snapshot-create" => {
    //            rvirsh::snapshot::main(&settings, &command)?
    //        }
    //        "pool-list" | "pool-info" | "pool-refresh" | "pool-uuid" | "pool-stop" | "pool-delete"
    //        | "pool-undefine" | "pool-clean" | "pool-autostart" | "pool-noautostart"
    //        | "pool-dumpxml" | "pool-start" | "pool-define" | "pool-create" => {
    //            rvirsh::pool::main(&settings, &command)?
    //        }
    //        "template-list" | "template-create" | "template-delete" | "template-info" | "spawn"
    //        | "deploy" => rvirsh::template::main(&settings, &command)?,
    //    };

    Ok(())
}

fn deprecated_err(dcmd: &Commands, pcmd: &str) -> Result<()> {
    bail!(format!("'{:?}' is deprecated. Use '{}'", dcmd, pcmd))
}

fn get_validate_fpath(path: &PathBuf) -> Result<PathBuf> {
    if path.exists() {
        Ok(path.to_owned())
    } else {
        bail!("Failed to locate config file; {:?}", path)
    }
}

fn get_config_file(config: &Option<PathBuf>) -> Result<PathBuf> {
    // config file is specified
    if let Some(config_file) = config {
        return get_validate_fpath(config_file);
    }

    // set default config file
    let config_file = if cfg!(debug_assertions) {
        let config_path = format!("{}/default.toml", env!("CARGO_MANIFEST_DIR"));
        println!("Using {}", config_path);
        PathBuf::from(config_path)
    } else {
        home::home_dir()
            .with_context(|| format!("Failed to locate home directory"))?
            .join(".config/rvirsh/default.toml")
    };

    get_validate_fpath(&config_file)
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
