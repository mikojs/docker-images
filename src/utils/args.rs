use std::fs;
use std::path::Path;

use clap::{Arg, ArgMatches};
use regex::Regex;

const HOSTNAME_PATH: &str = "/etc/hostname";

#[path = "../utils/main.rs"] mod utils;

pub fn set_proxy_arg() -> Arg<'static> {
    Arg::new("args")
        .required(true)
        .multiple_values(true)
        .allow_hyphen_values(true)
}

pub fn get_values_from_args(sub_matches: &ArgMatches) -> Vec<&str> {
    sub_matches
        .values_of("args")
        .unwrap()
        .collect()
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
    let cwd = utils::get_current_dir()
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
