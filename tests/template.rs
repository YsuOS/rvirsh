mod common;

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

#[test]
fn template_test() {
    let pool_name = "test-templates-pool";
    let pool_path = "/tmp/".to_string() + pool_name;
    let xml = &set_name_xml(pool_name, POOL_XML);
    let conn = Connect::open(Some(CONN)).unwrap();

    std::fs::create_dir_all(&pool_path).unwrap();
    rvirsh::pool::pool_create::create_pool(&conn, xml).unwrap();

    let pool = StoragePool::lookup_by_name(&conn, TEMP_POOL).unwrap();

    Command::cargo_bin("rv")
        .unwrap()
        .arg("template-list")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("{:<25}", "Name")));

    rvirsh::pool::pool_stop::stop_pool(&pool).unwrap();
    std::fs::remove_dir_all(&pool_path).unwrap();
}
