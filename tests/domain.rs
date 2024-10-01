mod common;

use assert_cmd::Command;
use common::*;
use predicates::prelude::*;
use virt::connect::Connect;

const XML: &str = r#"
<domain type="kvm">
  <name>NAME</name>
  <memory>1024</memory>
  <os>
    <type arch="x86_64" machine="q35">hvm</type>
  </os>
</domain>
"#;

#[test]
fn temporary_domain_test() {
    let vm_name = "test-vm";
    let xml = &set_xml(vm_name, XML);
    let conn = Connect::open(Some(CONN)).unwrap();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("create")
        .assert()
        .failure();

    assert!(rvirsh::domain::create::create_domain(&conn, xml).is_ok());

    Command::cargo_bin("rv")
        .unwrap()
        .arg("suspend")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Instance Name        State"));

    Command::cargo_bin("rv")
        .unwrap()
        .arg("info")
        .assert()
        .failure();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("dominfo")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("resume")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("poweroff")
        .arg(vm_name)
        .assert()
        .success();
}

#[test]
fn domain_test() {
    let vm_name = "test-vm1";
    let xml = &set_xml(vm_name, XML);
    let conn = Connect::open(Some(CONN)).unwrap();

    assert!(rvirsh::domain::define::define_domain(&conn, xml).is_ok());

    Command::cargo_bin("rv")
        .unwrap()
        .arg("suspend")
        .arg(vm_name)
        .assert()
        .failure();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("start")
        .assert()
        .failure();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("start")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("autostart")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("domid")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("dominfo")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("domuuid")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("domstate")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("dumpxml")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("info")
        .arg(vm_name)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{:<20} {}",
            "Autostart:", "enable"
        )));

    Command::cargo_bin("rv")
        .unwrap()
        .arg("noautostart")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("info")
        .arg(vm_name)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{:<20} {}",
            "Autostart:", "disable"
        )));

    Command::cargo_bin("rv")
        .unwrap()
        .arg("reboot")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("shutdown")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("reset")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("undefine")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("poweroff")
        .arg(vm_name)
        .assert()
        .success();
}
