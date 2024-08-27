use virt::domain::Domain;

pub fn show_domain_info(dom: &Domain) {
    let state = crate::domain::get_state_str(dom);
    let dominfo = dom.get_info().unwrap();
    let autostart = if dom.get_autostart().unwrap() {
        "enable"
    } else {
        "disable"
    };
    let managed_save = if dom.has_managed_save(0).unwrap() {
        "yes"
    } else {
        "no"
    };

    println!("{:<20} {}", "Id:", dom.get_id().unwrap());
    println!("{:<20} {}", "Name:", dom.get_name().unwrap());
    println!("{:<20} {}", "UUID:", dom.get_uuid_string().unwrap());
    println!("{:<20} {}", "OS Type:", dom.get_os_type().unwrap());
    println!("{:<20} {}", "State:", state);
    println!("{:<20} {}", "CPU(s):", dom.get_max_vcpus().unwrap());
    println!("{:<20} {}", "CPU time:", dominfo.cpu_time);
    println!(
        "{:<20} {} KiB",
        "Max memory:",
        dom.get_max_memory().unwrap()
    );
    println!("{:<20} {} KiB", "Used memory:", dominfo.memory);
    println!("{:<20} {}", "Autostart:", autostart);
    println!("{:<20} {}", "Managed save:", managed_save);
    // TODO: Add the following columns
    //println!("{:<20} {}", "Persistent:", todo!());
    //println!("{:<20} {}", "Security model:", todo!());
    //println!("{:<20} {}", "Security DOI:", todo!());
    //println!("{:<20} {}", "Security label:", todo!());
    //println!("{:<20} {}", "Messages:", todo!());
}
