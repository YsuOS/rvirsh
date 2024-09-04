use config::Config;
use virt::connect::Connect;

fn calc_version(version: u32) -> String {
    format!(
        "{}.{}.{}",
        version / 1000000,
        (version % 1000000) / 1000,
        version % 1000
    )
}

pub fn main(settings: &Config) {
    let uri = settings.get_string("URI").unwrap();
    let conn = Connect::open(Some(&uri)).unwrap();

    let lib_ver = calc_version(conn.get_lib_version().unwrap());
    let hyp_ver = calc_version(conn.get_hyp_version().unwrap());

    //println!("Compiled againt library: libvirt {}", lib_ver);
    println!("Using Library: libvirt {}", lib_ver);
    //println!("Using API: QEMU {}", lib_ver);
    println!("Running hypervisor: QEMU {}", hyp_ver);
}
