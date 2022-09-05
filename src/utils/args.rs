use std::env;
use std::fs;
use std::path::Path;

use clap::Arg;
use regex::Regex;

const HOSTNAME_PATH: &str = "/etc/hostname";

pub fn set_proxy_arg() -> Arg<'static> {
    Arg::new("args")
        .required(true)
        .multiple_values(true)
        .allow_hyphen_values(true)
}

pub fn get_container_name() -> String {
    if !Path::new(HOSTNAME_PATH).exists() {
        return "".to_string();
    }

    fs::read_to_string(HOSTNAME_PATH)
        .expect("Couldn't read the file")
        .replace("\n", "")
}

pub fn get_working_directory() -> String {
    let cwd = env::current_dir()
        .expect("Couldn't get the currenct directory")
        .display()
        .to_string();
    let is_project = Regex::new(r"^/project")
        .unwrap()
        .is_match(&cwd);

    if is_project {
        return cwd;
    }

    "/project".to_string()
}
