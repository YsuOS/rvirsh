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
