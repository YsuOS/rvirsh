mod common;

use assert_cmd::Command;
use common::*;
use predicates::prelude::*;
use virt::connect::Connect;

const XML: &str = r#"
<pool type='dir'>
  <name>NAME</name>
  <target>
	  <path>/tmp/NAME/</path>
  </target>
</pool>
"#;

#[test]
fn temporary_pool_test() {
    let pool_name = "test-pool";
    let pool_path = "/tmp/".to_string() + pool_name;
    let xml = &set_name_xml(pool_name, XML);
    let conn = Connect::open(Some(CONN)).unwrap();

    std::fs::create_dir_all(&pool_path).unwrap();

    assert!(rvirsh::pool::pool_create::create_pool(&conn, xml).is_ok());

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-start")
        .assert()
        .failure();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-stop")
        .arg(pool_name)
        .assert()
        .success();

    std::fs::remove_dir_all(&pool_path).unwrap();
}

#[test]
fn pool_test() {
    let pool_name = "test-pool1";
    let pool_path = "/tmp/".to_string() + pool_name;
    let xml = &set_name_xml(pool_name, XML);
    let conn = Connect::open(Some(CONN)).unwrap();

    std::fs::create_dir_all(&pool_path).unwrap();

    assert!(rvirsh::pool::pool_define::define_pool(&conn, xml).is_ok());

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-list")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{:<15} {:<10} {}",
            "Name", "State", "Autostart"
        )));

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-autostart")
        .arg(pool_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-info")
        .arg(pool_name)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{:<20} {}",
            "Autostart:", "true"
        )));

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-noautostart")
        .arg(pool_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-info")
        .arg(pool_name)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{:<20} {}",
            "Autostart:", "false"
        )));

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-uuid")
        .arg(pool_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-start")
        .arg(pool_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-refresh")
        .arg(pool_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-dumpxml")
        .arg(pool_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-stop")
        .arg(pool_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-delete")
        .arg(pool_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-undefine")
        .arg(pool_name)
        .assert()
        .success();

    std::fs::create_dir_all(&pool_path).unwrap();

    assert!(rvirsh::pool::pool_define::define_pool(&conn, xml).is_ok());

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-clean")
        .arg(pool_name)
        .assert()
        .success();
}
