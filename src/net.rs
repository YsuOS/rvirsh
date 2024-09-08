mod net_list;

use config::Config;
use virt::connect::Connect;

pub fn main(settings: &Config, cmd: &str) {
    let uri = settings.get_string("URI").unwrap();
    let conn = Connect::open(Some(&uri)).unwrap();

    if cmd == "net-list" {
        net_list::list_net(&conn);
        return;
    }
}
