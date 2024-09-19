use virt::{domain::Domain, domain_snapshot::DomainSnapshot};

pub fn get_current_snapshot(dom: &Domain) {
    let snapshot = DomainSnapshot::current(dom, 0).unwrap();

    crate::snapshot::snapshot_dumpxml::show_snapshot_dumpxml(&snapshot);
}
