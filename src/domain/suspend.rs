use virt::domain::Domain;

pub fn suspend_domain(dom: &Domain) {
    if !dom.is_active().unwrap() {
        println!("Domain {} is inactive", &dom.get_name().unwrap());
        return;
    }

    dom.suspend().unwrap();
    println!("Domain {} suspended", &dom.get_name().unwrap());
}
