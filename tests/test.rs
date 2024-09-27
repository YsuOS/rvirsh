use std::io::Write;

use assert_cmd::Command;
use predicates::prelude::*;

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
fn version() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("Using Library:"))
        .stdout(predicate::str::contains("Running hypervisor:"));
}

//TODO: Need test for get_config_file

#[test]
fn list() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Instance Name        State"));
}

#[test]
fn info_without_dom() {
    Command::cargo_bin("rv")
        .unwrap()
        .arg("info")
        .assert()
        .failure();
}

#[test]
fn temporary_domain_test() {
    let xml_path = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/test.xml";
    let vm_name = "test-vm";

    Command::cargo_bin("rv")
        .unwrap()
        .arg("run")
        .arg(&xml_path)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("suspend")
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
    let xml_path = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/test1.xml";
    let vm_name = "test-vm1";

    Command::cargo_bin("rv")
        .unwrap()
        .arg("define")
        .arg(&xml_path)
        .assert()
        .success();

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
        .arg("reset")
        .arg(vm_name)
        .assert()
        .success();

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
    let xml_path = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/test-net.xml";
    let net_name = "test-net";
    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-create")
        .arg(&xml_path)
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
    let xml_path = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/test-net1.xml";
    let net_name = "test-net1";

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-define")
        .arg(&xml_path)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-undefine")
        .arg(net_name)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("net-define")
        .arg(&xml_path)
        .assert()
        .success();

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
    let xml_path = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/test-vol.xml";
    let vol_name = "test-vol.qcow2";

    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-create")
        .arg(xml_path)
        .assert()
        .success();

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
    let xml_path = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/test-pool.xml";
    let pool_name = "test-pool";
    let pool_path = "/tmp/test-pool1";

    std::fs::create_dir_all(&pool_path).unwrap();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-create")
        .arg(&xml_path)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-stop")
        .arg(pool_name)
        .assert()
        .success();
}

#[test]
fn pool_test() {
    let xml_path = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/test-pool1.xml";
    let pool_name = "test-pool1";
    let pool_path = "/tmp/test-pool1";

    std::fs::create_dir_all(&pool_path).unwrap();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-define")
        .arg(&xml_path)
        .assert()
        .success();

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

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-define")
        .arg(&xml_path)
        .assert()
        .success();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("pool-clean")
        .arg(pool_name)
        .assert()
        .success();
}

fn set_filename(vol_path: &str, xml_path_src: &str, xml_path_dst: &str) {
    let xml_str = std::fs::read_to_string(&xml_path_src).unwrap();
    let xml_str = xml_str.replace("FILENAME", vol_path);
    let mut file = std::fs::File::create(&xml_path_dst).unwrap();
    write!(file, "{}", xml_str).unwrap();
}

#[test]
fn snapshot_test() {
    let vm_xml_path_src =
        env!("CARGO_MANIFEST_DIR").to_string() + "/resources/test-snapshot/test2.xml";
    let vm_xml_path_dst =
        env!("CARGO_MANIFEST_DIR").to_string() + "/resources/test-snapshot/test2-copy.xml";
    let vm_name = "test-vm2";

    let vol_xml_path =
        env!("CARGO_MANIFEST_DIR").to_string() + "/resources/test-snapshot/test-vol1.xml";
    let vol_name = "test-vm2.qcow2";

    let snapshot_name = "test-snapshot";
    let snapshot_child_name = "test-snapshot1";

    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-create")
        .arg(&vol_xml_path)
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
    set_filename(&vol_path, &vm_xml_path_src, &vm_xml_path_dst);

    Command::cargo_bin("rv")
        .unwrap()
        .arg("run")
        .arg(&vm_xml_path_dst)
        .assert()
        .success();

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

    std::fs::remove_file(vm_xml_path_dst).unwrap();
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
    let vm_xml_path_src =
        env!("CARGO_MANIFEST_DIR").to_string() + "/resources/test-delete/test3.xml";
    let vm_xml_path_dst =
        env!("CARGO_MANIFEST_DIR").to_string() + "/resources/test-delete/test3-copy.xml";
    let vm_name = "test-vm3";

    let vol_xml_path =
        env!("CARGO_MANIFEST_DIR").to_string() + "/resources/test-delete/test-vol2.xml";
    let vol_name = "test-vm3.qcow2";

    Command::cargo_bin("rv")
        .unwrap()
        .arg("vol-create")
        .arg(&vol_xml_path)
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
    set_filename(&vol_path, &vm_xml_path_src, &vm_xml_path_dst);

    Command::cargo_bin("rv")
        .unwrap()
        .arg("define")
        .arg(&vm_xml_path_dst)
        .assert()
        .success();

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

    std::fs::remove_file(vm_xml_path_dst).unwrap();
}
