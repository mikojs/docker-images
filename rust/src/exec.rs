use std::fs;
use std::path::Path;
use std::env;

use clap::{Command, Arg, ArgMatches};
use regex::Regex;

#[allow(dead_code)]
#[path = "./utils/sub_process.rs"] mod sub_process;

pub fn command() -> Command<'static> {
    Command::new("exec")
        .about(r#"This command would set the working directory with `docker exec`
When the current path is under `/project`, the same path would be the initial working directory
Otherwise, this would change to be `/project`"#)
        .arg(
            Arg::new("args")
                .required(true)
                .multiple_values(true)
                .allow_hyphen_values(true)
        )
}

fn get_working_directory() -> String {
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

pub fn execute(sub_matches: &ArgMatches) {
    let status = sub_process::exec(
        "docker",
        [
            vec![
                "exec",
                "-w",
                &get_working_directory(),
            ],
            sub_matches
                .values_of("args")
                .unwrap()
                .collect(),
        ].concat().as_slice(),
    );

    assert!(status.success());
}
