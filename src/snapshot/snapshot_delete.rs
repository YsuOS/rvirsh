use virt::domain::Domain;

pub fn delete_all_snapshots(dom: &Domain) {
    let snapshots = dom.list_all_snapshots(0).unwrap();

    for snapshot in snapshots {
        snapshot.delete(0).unwrap();
        println!("{} is deleted", snapshot.get_name().unwrap());
    }
    println!("All snapshots of {} is deleted", &dom.get_name().unwrap());
}
