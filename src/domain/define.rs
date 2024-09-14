use std::{fs::File, io::Read};
use virt::{connect::Connect, domain::Domain};

pub fn define_domain(conn: &Connect, xml: &mut File) {
    let mut content = String::new();
    xml.read_to_string(&mut content).unwrap();

    let dom = Domain::define_xml(conn, &content).unwrap();
    println!("Domain {} is defined", dom.get_name().unwrap());
}
