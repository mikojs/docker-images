use std::env;

use clap::Arg;
use regex::Regex;

pub fn set_proxy_arg() -> Arg<'static> {
    Arg::new("args")
        .required(true)
        .multiple_values(true)
        .allow_hyphen_values(true)
}

pub fn get_working_directory() -> String {
    let cwd = env::current_dir()
        .expect("Couldn't get the currenct directory")
        .display()
        .to_string();
    let re = Regex::new(r"^/project")
        .unwrap();

    if re.is_match(&cwd) {
        return cwd;
    }

    "/project".to_string()
}
