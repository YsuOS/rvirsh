use virt::{
    domain::Domain, domain_snapshot::DomainSnapshot, sys::VIR_DOMAIN_SNAPSHOT_LIST_DESCENDANTS,
};

pub fn show_snapshot_info(dom: &Domain, snapshot: &DomainSnapshot) {
    let current = if snapshot.is_current(0).unwrap() {
        "yes"
    } else {
        "no"
    };
    let metadata = if snapshot.has_metadata(0).unwrap() {
        "yes"
    } else {
        "no"
    };
    let parent_name = if snapshot.get_parent(0).is_ok() {
        let parent = snapshot.get_parent(0).unwrap();
        parent.get_name().unwrap()
    } else {
        "-".to_string()
    };

    println!("{:<15} {}", "Name:", snapshot.get_name().unwrap());
    println!("{:<15} {}", "Domain:", dom.get_name().unwrap());
    println!("{:<15} {}", "Current:", current);
    //println!("{:<15} {}", "State:", todo!());
    //println!("{:<15} {}", "Location:", todo!());
    println!("{:<15} {}", "Parent:", parent_name);
    println!("{:<15} {}", "Children:", snapshot.num_children(0).unwrap());
    println!(
        "{:<15} {}",
        "Descendants:",
        snapshot
            .num_children(VIR_DOMAIN_SNAPSHOT_LIST_DESCENDANTS)
            .unwrap()
    );
    println!("{:<15} {}", "Metadata:", metadata);
}
