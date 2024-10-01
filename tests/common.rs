// This test uses the following connection
pub const CONN: &str = "qemu:///system";

pub fn set_xml(name: &str, xml: &str) -> String {
    let xml = xml.replace("NAME", name);
    xml.to_string()
}
