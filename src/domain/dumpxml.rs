use virt::domain::Domain;

pub fn show_domain_dumpxml(dom: &Domain) {
    println!("{}", dom.get_xml_desc(0).unwrap());
}
