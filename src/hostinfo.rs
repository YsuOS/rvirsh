use anyhow::Result;
use config::Config;

use crate::get_conn;

pub fn main(settings: &Config) -> Result<()> {
    let conn = get_conn(settings)?;
    let nodeinfo = conn.get_node_info()?;

    println!("{:<20} {}", "CPU model:", nodeinfo.model);
    println!("{:<20} {}", "CPU(s):", nodeinfo.cpus);
    println!("{:<20} {} MHz", "CPU frequency:", nodeinfo.mhz);
    println!("{:<20} {}", "CPU socket(s):", nodeinfo.sockets);
    println!("{:<20} {}", "Core(s) per socket:", nodeinfo.cores);
    println!("{:<20} {}", "Thread(s) per core:", nodeinfo.threads);
    println!("{:<20} {}", "NUMA cell(s):", nodeinfo.nodes);
    println!("{:<20} {} KiB", "Memory size:", nodeinfo.memory);
    Ok(())
}
