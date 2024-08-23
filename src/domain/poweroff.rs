use virt::domain::Domain;

pub fn poweroff_domain(dom: &Domain) {
    if !dom.is_active().unwrap() {
        println!("Domain {} is inactive", &dom.get_name().unwrap());
        return;
    }

    dom.destroy().unwrap();
    println!("Domain {} is powered off", &dom.get_name().unwrap());
}
