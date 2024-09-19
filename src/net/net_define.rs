use std::{fs::File, io::Read};
use virt::{connect::Connect, network::Network};

pub fn define_net(conn: &Connect, xml: &mut File) {
    let mut content = String::new();
    xml.read_to_string(&mut content).unwrap();

    let net = Network::define_xml(conn, &content).unwrap();
    println!("{} is defined", net.get_name().unwrap());
}
