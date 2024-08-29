use virt::domain::Domain;

pub fn show_domain_id(dom: &Domain) {
    println!("{}", dom.get_id().unwrap());
}
