use anyhow::Result;
use config::Config;

use crate::get_conn;

pub fn main(settings: &Config) -> Result<()> {
    let conn = get_conn(settings)?;

    println!("{}", conn.get_hostname().unwrap());
    Ok(())
}
