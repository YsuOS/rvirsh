use virt::domain::Domain;

pub fn shutdown_domain(dom: &Domain) {
    if !dom.is_active().unwrap() {
        println!("Domain {} is inactive", &dom.get_name().unwrap());
        return;
    }

    dom.shutdown().unwrap();
    println!("Domain {} shutdowned", &dom.get_name().unwrap());
}
