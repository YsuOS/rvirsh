use virt::domain::Domain;

pub fn autostart_domain(dom: &Domain) {
    dom.set_autostart(true).unwrap();
    println!("{} sets autostart", dom.get_name().unwrap());
}
