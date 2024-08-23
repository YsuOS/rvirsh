use virt::domain::Domain;

pub fn create_domain(dom: &Domain) {
    dom.create().unwrap();
    println!("Domain {} started", &dom.get_name().unwrap());
}
