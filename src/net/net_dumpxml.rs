use virt::network::Network;

pub fn show_net_dumpxml(net: &Network) {
    println!("{}", net.get_xml_desc(0).unwrap());
}
