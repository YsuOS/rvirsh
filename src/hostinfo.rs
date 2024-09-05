use config::Config;
use virt::connect::Connect;

pub fn main(settings: &Config) {
    let uri = settings.get_string("URI").unwrap();
    let conn = Connect::open(Some(&uri)).unwrap();
    let nodeinfo = conn.get_node_info().unwrap();

    println!("{:<20} {}", "CPU model:", nodeinfo.model);
    println!("{:<20} {}", "CPU(s):", nodeinfo.cpus);
    println!("{:<20} {} MHz", "CPU frequency:", nodeinfo.mhz);
    println!("{:<20} {}", "CPU socket(s):", nodeinfo.sockets);
    println!("{:<20} {}", "Core(s) per socket:", nodeinfo.cores);
    println!("{:<20} {}", "Thread(s) per core:", nodeinfo.threads);
    println!("{:<20} {}", "NUMA cell(s):", nodeinfo.nodes);
    println!("{:<20} {} KiB", "Memory size:", nodeinfo.memory);
}
