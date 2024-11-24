mod common;

use assert_cmd::Command;
use common::*;
use predicates::prelude::*;
use virt::{connect::Connect, storage_pool::StoragePool};

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
    let xml = &set_name_xml(vm_name, XML);
    let conn = Connect::open(Some(CONN)).unwrap();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("create")
        .assert()
        .failure();

    assert!(rvirsh::domain::create_domain(&conn, xml).is_ok());

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
    let xml = &set_name_xml(vm_name, XML);
    let conn = Connect::open(Some(CONN)).unwrap();

    assert!(rvirsh::domain::define_domain(&conn, xml).is_ok());

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

const VM_XML: &str = r#"
<domain type="kvm">
  <name>test-vm4</name>
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
fn clone_test() {
    let org_vm_name = "test-vm4";
    let vol_name = "test-vm4.qcow2";
    let vol_xml = &set_name_xml(vol_name, VOL_XML);

    let conn = Connect::open(Some(CONN)).unwrap();
    let pool = StoragePool::lookup_by_name(&conn, POOL).unwrap();

    assert!(rvirsh::volume::create_vol(&pool, vol_xml).is_ok());

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
    let vm_xml = &set_name_xml(&org_vm_name, vm_xml);

    assert!(rvirsh::domain::define_domain(&conn, &vm_xml).is_ok());

    let new_vm_name = "test-vm4-clone";
    let new_vol_path = "/tmp/test-vm4-clone.qcow2";

    Command::cargo_bin("rv")
        .unwrap()
        .arg("clone")
        .arg(org_vm_name)
        .arg(new_vm_name)
        .arg(new_vol_path)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("delete")
        .arg(org_vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("delete")
        .arg(new_vm_name)
        .assert()
        .success();
}
