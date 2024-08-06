use virt::connect::Connect;

const URI: &str = "qemu:///system";

pub fn main() {
    let conn = Connect::open(Some(URI)).unwrap();
    let doms = conn.list_all_domains(0).unwrap();

    println!("Instance Name\tState");
    println!("-----------------------");
    for dom in doms {
        let name = dom.get_name().unwrap();
        let state = if dom.is_active().unwrap() {
            "running"
        } else {
            "shut off"
        };

        println!("{}\t{}", name, state);
    }

}
