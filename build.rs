use home;
use std::fs;

fn main() {
    let home_dir = home::home_dir().unwrap();
    let out_dir = home_dir.join(".config/rvirsh/");
    if !out_dir.exists() {
        fs::create_dir_all(&out_dir).unwrap();
    }

    let file = "default.toml";
    fs::copy(file, out_dir.join(file)).unwrap();
}
