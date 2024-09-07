use virt::domain_snapshot::DomainSnapshot;

pub fn show_snapshot_parent(snapshot: &DomainSnapshot) {
    if snapshot.get_parent(0).is_ok() {
        let parent = snapshot.get_parent(0).unwrap();
        println!("{}", parent.get_name().unwrap());
    }
}
