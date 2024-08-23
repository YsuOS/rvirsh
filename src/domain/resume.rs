use virt::{domain::Domain, sys::VIR_DOMAIN_PAUSED};

pub fn resume_domain(dom: &Domain) {
    let (state, _) = dom.get_state().unwrap();
    if state != VIR_DOMAIN_PAUSED {
        println!("Domain {} is not paused", &dom.get_name().unwrap());
        return;
    }

    dom.resume().unwrap();
    println!("Domain {} resumed", &dom.get_name().unwrap());
}
