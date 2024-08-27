use virt::domain::Domain;

pub fn reset_domain(dom: &Domain) {
    if !dom.is_active().unwrap() {
        println!("Domain {} is inactive", &dom.get_name().unwrap());
        return;
    }

    dom.reset().unwrap();
    println!("Domain {} is reset", &dom.get_name().unwrap());
}
