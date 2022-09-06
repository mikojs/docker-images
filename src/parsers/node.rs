use std::env;
use std::path::{Path, PathBuf};

fn find_package_json(cwd: PathBuf) -> PathBuf {
    let file_path = cwd.join("package.json");

    if !file_path.exists() {
        if cwd == Path::new("/") {
            return Path::new("")
                .to_path_buf();
        }

        return find_package_json(
            cwd.parent()
                .expect("couldn't find the parent directory")
                .to_path_buf(),
        );
    }

    file_path
}

fn main() {
    let cwd = env::current_dir()
        .expect("Couldn't get the currenct directory");
    let package_json_path = find_package_json(cwd)
        .display()
        .to_string();

    if package_json_path.is_empty() {
        return;
    }

    println!("{}", package_json_path);
}
