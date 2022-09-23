use std::fs;
use std::path::Path;

const HOSTNAME_PATH: &str = "/etc/hostname";

pub fn main() -> String {
    if !Path::new(HOSTNAME_PATH).exists() {
        return "".to_string();
    }

    fs::read_to_string(HOSTNAME_PATH)
        .expect("Couldn't read the file")
        .replace("\n", "")
}
