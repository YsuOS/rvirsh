use anyhow::Result;
use virt::domain::Domain;

pub fn show_domain_dumpxml(dom: &Domain) -> Result<()> {
    println!("{}", dom.get_xml_desc(0).unwrap());
    Ok(())
}
