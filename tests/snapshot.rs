mod common;

use assert_cmd::Command;
use common::*;
use predicates::prelude::*;
use virt::{connect::Connect, storage_pool::StoragePool};

const VM_XML: &str = r#"
<domain type="kvm">
  <name>test-vm2</name>
  <memory>1024</memory>
  <os>
    <type arch="x86_64" machine="q35">hvm</type>
  </os>
  <devices>
    <disk type="file" device="disk">
      <driver name="qemu" type="qcow2"/>
      <source file="FILENAME"/>
      <target dev="vda" bus="virtio"/>
    </disk>
  </devices>
</domain>
"#;

const VOL_XML: &str = r#"
<volume>
  <name>NAME</name>
  <capacity>1</capacity>
  <target>
    <format type='qcow2'/>
  </target>
</volume>
"#;

#[test]
fn snapshot_test() {
    let vm_name = "test-vm2";

    let snapshot_name = "test-snapshot";
    let snapshot_child_name = "test-snapshot1";

    let vol_name = "test-vm2.qcow2";
    let vol_xml = &set_name_xml(vol_name, VOL_XML);

    let conn = Connect::open(Some(CONN)).unwrap();
    let pool = StoragePool::lookup_by_name(&conn, POOL).unwrap();

    assert!(rvirsh::volume::vol_create::create_vol(&pool, vol_xml).is_ok());

    let output = Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-path")
        .arg(vol_name)
        .assert()
        .success()
        .get_output()
        .clone();

    let vol_path = get_vol_path(output);
    let vm_xml = &set_fname_xml(&vol_path, VM_XML);
    let vm_xml = &set_name_xml(&vm_name, vm_xml);

    assert!(rvirsh::domain::create::create_domain(&conn, &vm_xml).is_ok());

    Command::cargo_bin("rv")
        .unwrap()
        .arg("snapshot-info")
        .arg(vm_name)
        .assert()
        .failure();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("snapshot-list")
        .arg(vm_name)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("{:<10}", "Name")));

    Command::cargo_bin("rv")
        .unwrap()
        .arg("snapshot-create")
        .arg(vm_name)
        .arg(snapshot_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("snapshot-current")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("snapshot-dumpxml")
        .arg(vm_name)
        .arg(snapshot_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("snapshot-create")
        .arg(vm_name)
        .arg(snapshot_child_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("snapshot-parent")
        .arg(vm_name)
        .arg(snapshot_child_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("snapshot-info")
        .arg(vm_name)
        .arg(snapshot_child_name)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{:<15} {}",
            "Parent:", snapshot_name
        )));

    Command::cargo_bin("rv")
        .unwrap()
        .arg("snapshot-revert")
        .arg(vm_name)
        .arg(snapshot_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("snapshot-delete")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("poweroff")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-delete")
        .arg(vol_name)
        .assert()
        .success();
}
