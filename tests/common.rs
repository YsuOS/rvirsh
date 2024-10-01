#![allow(dead_code)]

use std::process::Output;

// This test uses the following connection and pool
pub const CONN: &str = "qemu:///system";
pub const POOL: &str = "default";

pub fn set_name_xml(name: &str, xml: &str) -> String {
    let xml = set_xml(name, xml, "NAME");
    xml.to_string()
}

pub fn set_fname_xml(name: &str, xml: &str) -> String {
    let xml = set_xml(name, xml, "FILENAME");
    xml.to_string()
}

fn set_xml(name: &str, xml: &str, dst: &str) -> String {
    let xml = xml.replace(dst, name);
    xml.to_string()
}
pub fn get_vol_path(output: Output) -> String {
    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout.lines().nth(1).unwrap().to_string()
}
