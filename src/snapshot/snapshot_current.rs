use virt::{domain::Domain, domain_snapshot::DomainSnapshot};

pub fn get_current_snapshot(dom: &Domain) {
    let snapshot = DomainSnapshot::current(dom, 0).unwrap();

    println!("{}", snapshot.get_name().unwrap());
}
