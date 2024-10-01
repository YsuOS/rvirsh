#![allow(dead_code)]

use std::process::Output;

// This test uses the following connection and pool
pub const CONN: &str = "qemu:///system";
pub const POOL: &str = "default";

pub fn set_xml(name: &str, xml: &str) -> String {
    let xml = xml.replace("NAME", name);
    xml.to_string()
}

pub fn get_vol_path(output: Output) -> String {
    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout.lines().nth(1).unwrap().to_string()
}
