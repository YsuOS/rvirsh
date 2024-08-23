use virt::domain::Domain;

pub fn reboot_domain(dom: &Domain) {
    if !dom.is_active().unwrap() {
        println!("Domain {} is inactive", &dom.get_name().unwrap());
        return;
    }

    dom.reboot(0).unwrap();
    println!("Domain {} rebooted", &dom.get_name().unwrap());
}
