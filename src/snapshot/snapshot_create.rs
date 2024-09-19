use virt::{domain::Domain, domain_snapshot::DomainSnapshot};

pub fn create_snapshot(dom: &Domain) {
    let xml = "<domainsnapshot/>";
    let snapshot = DomainSnapshot::create_xml(dom, xml, 0).unwrap();

    println!(
        "{} is created on {}",
        snapshot.get_name().unwrap(),
        dom.get_name().unwrap()
    );
}
