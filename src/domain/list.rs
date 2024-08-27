use virt::connect::Connect;

pub fn list_domain(conn: &Connect) {
    let doms = conn.list_all_domains(0).unwrap();

    println!("{:<20} {}", "Instance Name", "State");
    println!("{:<20} {}", "-".repeat(20), "-".repeat(10));
    for dom in doms {
        let name = dom.get_name().unwrap();
        let state = crate::domain::get_state_str(&dom);
        println!("{:<20} {}", name, state);
    }
}
