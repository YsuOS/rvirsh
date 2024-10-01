use assert_cmd::Command;
use predicates::prelude::*;
use virt::{connect::Connect, storage_pool::StoragePool};

// This test uses the following connection and pool
const CONN: &str = "qemu:///system";
const POOL: &str = "default";

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

#[test]
fn help_test() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("help")
        .assert()
        .success();
}

#[test]
fn get_domain_test() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("start")
        .assert()
        .failure();
}

#[test]
fn get_xml_test() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("create")
        .assert()
        .failure();
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
fn get_net_name_test() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-info")
        .assert()
        .failure();
}

#[test]
fn net_list() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-list")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{:<10} {:<8} {:<10} {}",
            "Name", "State", "Autostart", "Persistent"
        )));
}

#[test]
fn temporary_net_test() {
    let xml = r#"
<network>
  <name>test-net</name>
</network>
"#;

    let net_name = "test-net";
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
        .arg("net-stop")
        .arg(net_name)
        .assert()
        .success();
}

#[test]
fn net_test() {
    let xml = r#"
<network>
  <name>test-net1</name>
</network>
"#;

    let net_name = "test-net1";
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

#[test]
fn vol_list() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-list")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{:<25} {}",
            "Name", "Path"
        )));
}

#[test]
fn volume_test() {
    let xml = r#"
<volume>
  <name>test-vol.qcow2</name>
  <capacity>1</capacity>
</volume>
"#;

    let vol_name = "test-vol.qcow2";
    let conn = Connect::open(Some(CONN)).unwrap();
    let pool = StoragePool::lookup_by_name(&conn, POOL).unwrap();

    assert!(rvirsh::volume::vol_create::create_vol(&pool, xml).is_ok());

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

    let vol_path = {
        let stdout = String::from_utf8(output.stdout).unwrap();
        stdout.lines().nth(1).unwrap().to_string()
    };

    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-pool")
        .assert()
        .failure();

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

#[test]
fn get_pool_name_test() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-start")
        .assert()
        .failure();
}

#[test]
fn pool_list() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-list")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{:<15} {:<10} {}",
            "Name", "State", "Autostart"
        )));
}

#[test]
fn temporary_pool_test() {
    let xml = r#"
<pool type='dir'>
  <name>test-pool</name>
  <target>
	  <path>/tmp/test-pool/</path>
  </target>
</pool>
"#;

    let pool_name = "test-pool";
    let pool_path = "/tmp/test-pool";
    let conn = Connect::open(Some(CONN)).unwrap();

    std::fs::create_dir_all(&pool_path).unwrap();

    assert!(rvirsh::pool::pool_create::create_pool(&conn, xml).is_ok());

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-stop")
        .arg(pool_name)
        .assert()
        .success();
}

#[test]
fn pool_test() {
    let xml = r#"
<pool type='dir'>
  <name>test-pool1</name>
  <target>
	  <path>/tmp/test-pool1</path>
  </target>
</pool>
"#;

    let pool_name = "test-pool1";
    let pool_path = "/tmp/test-pool1";
    let conn = Connect::open(Some(CONN)).unwrap();

    std::fs::create_dir_all(&pool_path).unwrap();

    assert!(rvirsh::pool::pool_define::define_pool(&conn, xml).is_ok());

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

#[test]
fn snapshot_test() {
    let vm_xml = r#"
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
    let vm_name = "test-vm2";
    let vol_xml = r#"
<volume>
  <name>test-vm2.qcow2</name>
  <capacity>1</capacity>
  <target>
    <format type='qcow2'/>
  </target>
</volume>
"#;

    let snapshot_name = "test-snapshot";
    let snapshot_child_name = "test-snapshot1";

    let vol_name = "test-vm2.qcow2";
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

    let vol_path = {
        let stdout = String::from_utf8(output.stdout).unwrap();
        stdout.lines().nth(1).unwrap().to_string()
    };
    let vm_xml = vm_xml.replace("FILENAME", &vol_path);

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

#[test]
fn delete_test() {
    let vm_xml = r#"
<domain type="kvm">
  <name>test-vm3</name>
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
    let vm_name = "test-vm3";

    let vol_xml = r#"
<volume>
  <name>test-vm3.qcow2</name>
  <capacity>1</capacity>
  <target>
    <format type='qcow2'/>
  </target>
</volume>
"#;

    let vol_name = "test-vm3.qcow2";
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

    let vol_path = {
        let stdout = String::from_utf8(output.stdout).unwrap();
        stdout.lines().nth(1).unwrap().to_string()
    };
    let vm_xml = vm_xml.replace("FILENAME", &vol_path);

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
