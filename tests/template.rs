mod common;

use std::{fs::File, io::Write, path::Path};

use assert_cmd::{output::OutputOkExt, Command};
use common::*;
use predicates::prelude::*;
use virt::{connect::Connect, storage_pool::StoragePool, storage_vol::StorageVol};

const POOL_XML: &str = r#"
<pool type='dir'>
  <name>NAME</name>
  <target>
	  <path>/tmp/NAME/</path>
  </target>
</pool>
"#;

const VM_XML: &str = r#"
<domain type="kvm">
  <name>XXXX</name>
  <memory>1024</memory>
  <os>
    <type arch="x86_64" machine="q35">hvm</type>
  </os>
  <devices>
    <disk type="file" device="disk">
      <driver name="qemu" type="qcow2"/>
      <source file="XXXX"/>
      <target dev="vda" bus="virtio"/>
    </disk>
  </devices>
</domain>
"#;

#[test]
fn template_test() {
    let pool_name = "test-templates";
    let pool_path = "/tmp/".to_string() + pool_name;
    let xml = &set_name_xml(pool_name, POOL_XML);
    let conn = Connect::open(Some(CONN)).unwrap();

    if let Ok(pool) = StoragePool::lookup_by_name(&conn, pool_name) {
        rvirsh::pool::stop_pool(&pool).unwrap();
        std::fs::remove_dir_all(&pool_path).unwrap();
    }
    std::fs::create_dir_all(&pool_path).unwrap();
    rvirsh::pool::create_pool(&conn, xml).unwrap();

    let tmp_pool = StoragePool::lookup_by_name(&conn, pool_name).unwrap();

    let mut xml = File::create(Path::new("/tmp/test.xml")).unwrap();
    xml.write_all(VM_XML.as_bytes()).unwrap();

    std::process::Command::new("qemu-img")
        .arg("create")
        .arg("-f")
        .arg("qcow2")
        .arg("/tmp/test.qcow2")
        .arg("1K")
        .unwrap();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("template-create")
        .arg("test")
        .arg("/tmp/test.xml")
        .arg("/tmp/test.qcow2")
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("template-list")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("{:<25}", "Name")));

    let new_domain = "test-clone";
    let new_domain_vol = new_domain.to_string() + ".qcow2";
    let pool = StoragePool::lookup_by_name(&conn, "default").unwrap();
    if let Ok(vol) = StorageVol::lookup_by_name(&pool, &new_domain_vol) {
        rvirsh::volume::delete_volume(&vol).unwrap();
    }

    Command::cargo_bin("rv")
        .unwrap()
        .arg("spawn")
        .arg("test")
        .arg(new_domain)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("poweroff")
        .arg(new_domain)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-delete")
        .arg(&new_domain_vol)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("deploy")
        .arg("test")
        .arg(new_domain)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("delete")
        .arg(new_domain)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("template-info")
        .arg("test")
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("template-delete")
        .arg("test")
        .assert()
        .success();

    rvirsh::pool::stop_pool(&tmp_pool).unwrap();
    std::fs::remove_dir_all(&pool_path).unwrap();
    std::fs::remove_file(Path::new("/tmp/test.xml")).unwrap();
    std::fs::remove_file(Path::new("/tmp/test.qcow2")).unwrap();
}
