use std::fs;
use std::path::Path;

use clap::{Arg, ArgMatches};
use regex::Regex;

const HOSTNAME_PATH: &str = "/etc/hostname";

#[path = "../utils/main.rs"] mod utils;

pub fn set_proxy_arg(required: bool) -> Arg<'static> {
    Arg::new("args")
        .required(required)
        .multiple_values(true)
        .allow_hyphen_values(true)
}

pub fn get_values_from_args(sub_matches: &ArgMatches) -> Vec<&str> {
    let args = sub_matches
        .values_of("args");

    if args.is_none() {
        return vec![];
    }

    args
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
    let is_root = Regex::new(r"^/root")
        .unwrap()
        .is_match(&cwd);

    if is_root {
        return cwd;
    }

    "/root".to_string()
}
