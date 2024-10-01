mod common;

use assert_cmd::Command;
use common::*;
use predicates::prelude::*;
use virt::{connect::Connect, storage_pool::StoragePool};

#[test]
fn without_command() {
    Command::cargo_bin("rv")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::eq(
            "Error: 1st argument is required\nRun 'rv help' to see commands\n",
        ));
}

#[test]
fn unsupported_command() {
    let cmd = "unsupported";
    Command::cargo_bin("rv")
        .unwrap()
        .arg(cmd)
        .assert()
        .failure()
        .stderr(predicate::eq(format!(
            "Error: Command {} is not supported.\nRun 'rv help' to see commands\n",
            cmd
        )));
}

#[test]
fn deprecated_test() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("nodeinfo")
        .assert()
        .failure();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-destroy")
        .assert()
        .failure();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-destroy")
        .assert()
        .failure();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("destroy")
        .assert()
        .failure();
}

// Misc command tests
#[test]
fn help_test() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("help")
        .assert()
        .success();
}

#[test]
fn version() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("Using Library:"))
        .stdout(predicate::str::contains("Running hypervisor:"));
}

#[test]
fn uri_test() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("uri")
        .assert()
        .success();
}

#[test]
fn hostname_test() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("hostname")
        .assert()
        .success();
}

#[test]
fn hostinfo_test() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("hostinfo")
        .assert()
        .success();
}

const VM_XML: &str = r#"
<domain type="kvm">
  <name>NAME</name>
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
fn delete_test() {
    let vm_name = "test-vm3";

    let vol_name = "test-vm3.qcow2";
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

    assert!(rvirsh::domain::define::define_domain(&conn, &vm_xml).is_ok());

    Command::cargo_bin("rv")
        .unwrap()
        .arg("start")
        .arg(vm_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("delete")
        .arg(vm_name)
        .assert()
        .success();
}
