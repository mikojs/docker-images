use std::fs;
use std::path::Path;

use clap::{Arg, ArgMatches};
use regex::Regex;

const HOSTNAME_PATH: &str = "/etc/hostname";

#[path = "./get_current_dir.rs"] mod get_current_dir;

pub fn set_proxy_arg(required: bool) -> Arg<'static> {
    Arg::new("args")
        .required(required)
        .multiple_values(true)
        .allow_hyphen_values(true)
}

pub fn get_values_from_args(matches: &ArgMatches) -> Vec<&str> {
    let args = matches
        .values_of("args");

    if args.is_none() {
        return vec![];
    }

    args
        .unwrap()
        .collect()
}

pub fn filter_args(args: Vec<&str>) -> Vec<&str> {
    if args[1].is_empty() {
        return vec![];
    }

    args
}

pub fn get_working_directory() -> String {
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
