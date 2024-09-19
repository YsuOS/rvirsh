use virt::{
    domain::Domain,
    domain_snapshot::DomainSnapshot,
    sys::{VIR_DOMAIN_SNAPSHOT_CREATE_CURRENT, VIR_DOMAIN_SNAPSHOT_CREATE_REDEFINE},
};

pub fn set_snapshot(dom: &Domain, snapshot: &DomainSnapshot) {
    let xml = snapshot.get_xml_desc(0).unwrap();
    let _ = DomainSnapshot::create_xml(
        dom,
        &xml,
        VIR_DOMAIN_SNAPSHOT_CREATE_REDEFINE | VIR_DOMAIN_SNAPSHOT_CREATE_CURRENT,
    );

    println!(
        "{} is set on {}",
        snapshot.get_name().unwrap(),
        dom.get_name().unwrap()
    );
}
