use anyhow::Result;
use virt::{
    domain_snapshot::DomainSnapshot,
    sys::{
        VIR_DOMAIN_SNAPSHOT_CREATE_CURRENT, VIR_DOMAIN_SNAPSHOT_CREATE_REDEFINE,
        VIR_DOMAIN_SNAPSHOT_REVERT_RUNNING,
    },
};

pub fn revert_snapshot(snapshot: &DomainSnapshot) -> Result<()> {
    let xml = snapshot.get_xml_desc(0)?;
    let dom = snapshot.get_domain()?;
    let _ = DomainSnapshot::create_xml(
        &dom,
        &xml,
        VIR_DOMAIN_SNAPSHOT_CREATE_REDEFINE | VIR_DOMAIN_SNAPSHOT_CREATE_CURRENT,
    );

    snapshot.revert(VIR_DOMAIN_SNAPSHOT_REVERT_RUNNING)?;

    println!("{} is set on {}", snapshot.get_name()?, dom.get_name()?);
    Ok(())
}
