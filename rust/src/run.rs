use std::fs;
use std::path::Path;
use std::env;

use clap::{Command, Arg, ArgMatches};
use regex::Regex;

#[allow(dead_code)]
#[path = "./utils/sub_process.rs"] mod sub_process;

pub fn command() -> Command<'static> {
    Command::new("run")
        .about(r#"This command would mount the same volumes to the current container
When the current path is `/project`, a new container would use the same path.
Otherwise, this would change to be `/project`"#)
        .arg(
            Arg::new("args")
                .required(true)
                .multiple_values(true)
                .allow_hyphen_values(true)
        )
}

fn get_volumes_from_args(file_path: &str) -> Vec<String> {
    let mut args: Vec<String> = []
        .to_vec();

    if Path::new(file_path).exists() {
        let content = fs::read_to_string(file_path)
            .expect("Couldn't read the fale")
            .replace("\n", "");

        args.push("--volumes-from".to_string());
        args.push(content);
    }

    args
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
                "run",
                "-w",
                &get_working_directory(),
            ],
            get_volumes_from_args("/etc/hostname")
                .iter()
                .map(AsRef::as_ref)
                .collect(),
            sub_matches
                .values_of("args")
                .unwrap()
                .collect(),
        ].concat().as_slice(),
    );

    assert!(status.success());
}
