use crate::URI;
use virt::connect::Connect;

pub fn main() {
    let conn = Connect::open(Some(URI)).unwrap();
    let doms = conn.list_all_domains(0).unwrap();

    println!("{:<20} {}", "Instance Name", "State");
    println!("{:<20} {}", "--------------------", "--------");
    for dom in doms {
        let name = dom.get_name().unwrap();
        let state = if dom.is_active().unwrap() {
            "running"
        } else {
            "shut off"
        };

        println!("{:<20} {}", name, state);
    }
}
