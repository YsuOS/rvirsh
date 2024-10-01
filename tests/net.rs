mod common;

use assert_cmd::Command;
use common::*;
use predicates::prelude::*;
use virt::connect::Connect;

const XML: &str = r#"
<network>
  <name>NAME</name>
</network>
"#;

#[test]
fn temporary_net_test() {
    let net_name = "test-net";
    let xml = &set_name_xml(net_name, XML);
    let conn = Connect::open(Some(CONN)).unwrap();

    assert!(rvirsh::net::net_create::create_net(&conn, xml).is_ok());

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-info")
        .arg(net_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-info")
        .assert()
        .failure();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-stop")
        .arg(net_name)
        .assert()
        .success();
}

#[test]
fn net_test() {
    let net_name = "test-net1";
    let xml = &set_name_xml(net_name, XML);
    let conn = Connect::open(Some(CONN)).unwrap();

    assert!(rvirsh::net::net_define::define_net(&conn, xml).is_ok());

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-undefine")
        .arg(net_name)
        .assert()
        .success();

    assert!(rvirsh::net::net_define::define_net(&conn, xml).is_ok());

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-list")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{:<10} {:<8} {:<10} {}",
            "Name", "State", "Autostart", "Persistent"
        )));

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-start")
        .arg(net_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-autostart")
        .arg(net_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-uuid")
        .arg(net_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-info")
        .arg(net_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-dumpxml")
        .arg(net_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-info")
        .arg(net_name)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{:<15} {}",
            "Autostart:", "yes"
        )));

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-noautostart")
        .arg(net_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-info")
        .arg(net_name)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{:<15} {}",
            "Autostart:", "no"
        )));

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-clean")
        .arg(net_name)
        .assert()
        .success();
}
