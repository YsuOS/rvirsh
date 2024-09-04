use config::Config;
use virt::connect::Connect;

pub fn main(settings: &Config) {
    let uri = settings.get_string("URI").unwrap();
    let conn = Connect::open(Some(&uri)).unwrap();

    println!("{}", conn.get_hostname().unwrap());
}
