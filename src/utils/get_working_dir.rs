use regex::Regex;

#[path = "./get_current_dir.rs"] mod get_current_dir;

pub fn main() -> String {
    let cwd = get_current_dir::main()
        .display()
        .to_string();
    let is_root = Regex::new(r"^/root")
        .unwrap()
        .is_match(&cwd);

    if is_root {
        return cwd;
    }

    "/root".to_string()
}
