use config::Config;
use virt::connect::Connect;

pub fn main(settings: &Config) {
    let uri = settings.get_string("URI").unwrap();
    let conn = Connect::open(Some(&uri)).unwrap();
    let pools = conn.list_all_storage_pools(0).unwrap();

    let n_width = 15;
    let s_width = 10;
    println!("{:<n_width$} {:<s_width$} {}", "Name", "State", "Autostart");
    println!(
        "{:<n_width$} {:<s_width$} {}",
        "---------------", "----------", "---------------"
    );
    for pool in pools {
        let name = pool.get_name().unwrap();
        let state = if pool.is_active().unwrap() {
            "active"
        } else {
            "inactive"
        };
        let autostart = if pool.get_autostart().unwrap() {
            "yes"
        } else {
            "no"
        };

        println!("{:<n_width$} {:<s_width$} {}", name, state, autostart);
    }
}
