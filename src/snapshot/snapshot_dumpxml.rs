use virt::domain_snapshot::DomainSnapshot;

pub fn show_snapshot_dumpxml(snapshot: &DomainSnapshot) {
    println!("{}", snapshot.get_xml_desc(0).unwrap());
}
