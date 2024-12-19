use anyhow::{bail, Context, Result};
use clap::Parser;
use config::Config;
use rvirsh::{
    parser,
    Commands::{self, *},
};
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
        List | Start(_) | Shutdown(_) | Reboot(_) | Suspend(_) | Resume(_) | Reset(_)
        | Poweroff(_) | Undefine(_) | Dominfo(_) | Info(_) | Domid(_) | Domuuid(_)
        | Autostart(_) | Noautostart(_) | Domstate(_) | Dumpxml(_) | Define(_) | Create(_) => {
            rvirsh::domain::main(&settings, &command)?
        }
        Clone(_) => rvirsh::clone::main(&settings, &command)?,
        Delete(_) => rvirsh::delete::main(&settings, &command)?,
        Console(_) => rvirsh::console::main(&settings, &command)?,
        NetList | NetUuid(_) | NetInfo(_) | NetDumpxml(_) | NetAutostart(_) | NetNoautostart(_)
        | NetStop(_) | NetStart(_) | NetUndefine(_) | NetClean(_) | NetDefine(_) | NetCreate(_) => {
            rvirsh::net::main(&settings, &command)?
        }
        VolDelete(_) | VolList | VolInfo(_) | VolPath(_) | VolKey(_) | VolDumpxml(_)
        | VolPool(_) | VolWipe(_) | VolCreate(_) | VolClone(_) => {
            rvirsh::volume::main(&settings, &command)?
        }
        SnapshotList(_) | SnapshotDelete(_) | SnapshotInfo(_) | SnapshotParent(_)
        | SnapshotDumpxml(_) | SnapshotCurrent(_) | SnapshotRevert(_) | SnapshotCreate(_) => {
            rvirsh::snapshot::main(&settings, &command)?
        }
        PoolList | PoolInfo(_) | PoolRefresh(_) | PoolUuid(_) | PoolStop(_) | PoolDelete(_)
        | PoolUndefine(_) | PoolClean(_) | PoolAutostart(_) | PoolNoautostart(_)
        | PoolDumpxml(_) | PoolStart(_) | PoolDefine(_) | PoolCreate(_) => {
            rvirsh::pool::main(&settings, &command)?
        }
        TemplateList | TemplateCreate(_) | TemplateDelete(_) | TemplateInfo(_) | Spawn(_)
        | Deploy(_) => rvirsh::template::main(&settings, &command)?,
        Version => rvirsh::version::main(&settings)?,
        Uri => rvirsh::uri::main(&settings)?,
        Hostname => rvirsh::hostname::main(&settings)?,
        Hostinfo => rvirsh::hostinfo::main(&settings)?,
        Nodeinfo => deprecated_err(&command, "hostinfo")?,
        Destroy => deprecated_err(&command, "poweroff")?,
        NetDestroy => deprecated_err(&command, "net-stop")?,
        PoolDestroy => deprecated_err(&command, "pool-stop")?,
    };

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
