use config::Config;
use virt::{connect::Connect, storage_pool::StoragePool};

pub fn main(settings: &Config) {
    let uri = settings.get_string("URI").unwrap();
    let conn = Connect::open(Some(&uri)).unwrap();
    let pool_name = settings.get_string("POOL").unwrap();

    let pool = StoragePool::lookup_by_name(&conn, &pool_name).unwrap();
    let vols = pool.list_all_volumes(0).unwrap();

    let width = 25;
    println!("{:<width$} {}", "Name", "Path");
    println!(
        "{:<width$} {}",
        "-".repeat(25), "-".repeat(35)
    );
    for vol in vols {
        let name = vol.get_name().unwrap();
        let path = vol.get_path().unwrap();
        println!("{:<width$} {}", name, path);
    }
}
