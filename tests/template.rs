mod common;

use std::{fs::File, io::Write, path::Path};

use assert_cmd::Command;
use common::*;
use predicates::prelude::*;
use virt::{connect::Connect, storage_pool::StoragePool};

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
        rvirsh::pool::pool_stop::stop_pool(&pool).unwrap();
    }
    std::fs::create_dir_all(&pool_path).unwrap();
    rvirsh::pool::pool_create::create_pool(&conn, xml).unwrap();

    let pool = StoragePool::lookup_by_name(&conn, pool_name).unwrap();

    let mut xml = File::create(Path::new("/tmp/test.xml")).unwrap();
    xml.write_all(VM_XML.as_bytes()).unwrap();

    File::create(Path::new("/tmp/test.qcow2")).unwrap();

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

    std::fs::remove_file(Path::new("/tmp/test.xml")).unwrap();
    std::fs::remove_file(Path::new("/tmp/test.qcow2")).unwrap();
    rvirsh::pool::pool_stop::stop_pool(&pool).unwrap();
    std::fs::remove_dir_all(&pool_path).unwrap();
}
