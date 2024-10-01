mod common;

use assert_cmd::Command;
use common::*;
use predicates::prelude::*;
use virt::{connect::Connect, storage_pool::StoragePool};

const XML: &str = r#"
<volume>
  <name>NAME</name>
  <capacity>1</capacity>
</volume>
"#;

#[test]
fn volume_test() {
    let vol_name = "test-vol.qcow2";
    let xml = &set_xml(vol_name, XML);
    let conn = Connect::open(Some(CONN)).unwrap();
    let pool = StoragePool::lookup_by_name(&conn, POOL).unwrap();

    assert!(rvirsh::volume::vol_create::create_vol(&pool, xml).is_ok());

    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-list")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{:<25} {}",
            "Name", "Path"
        )));

    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-info")
        .assert()
        .failure();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-info")
        .arg(vol_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-dumpxml")
        .arg(vol_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-key")
        .arg(vol_name)
        .assert()
        .success();

    let output = Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-path")
        .arg(vol_name)
        .assert()
        .success()
        .get_output()
        .clone();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-pool")
        .assert()
        .failure();

    let vol_path = get_vol_path(output);
    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-pool")
        .arg(vol_path)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-wipe")
        .arg(vol_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-delete")
        .arg(vol_name)
        .assert()
        .success();
}
