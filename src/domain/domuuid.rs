use virt::domain::Domain;

pub fn show_domain_uuid(dom: &Domain) {
    println!("{}", dom.get_uuid().unwrap());
}
