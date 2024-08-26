use virt::storage_pool::StoragePool;

pub fn list_volume(pool: &StoragePool) {
    let vols = pool.list_all_volumes(0).unwrap();

    let width = 25;
    println!("{:<width$} {}", "Name", "Path");
    println!("{:<width$} {}", "-".repeat(25), "-".repeat(35));
    for vol in vols {
        let name = vol.get_name().unwrap();
        let path = vol.get_path().unwrap();
        println!("{:<width$} {}", name, path);
    }
}
