use regex::Regex;

use crate::utils::get_current_dir;

pub fn main() -> String {
    let cwd = get_current_dir::main()
        .display()
        .to_string();
    let is_work = Regex::new(r"^/root/work")
        .unwrap()
        .is_match(&cwd);

    if is_work {
        return cwd;
    }

    "/root/work".to_string()
}
