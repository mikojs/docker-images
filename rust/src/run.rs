use std::fs;
use std::path::Path;

use clap::{Command, ArgMatches};

#[allow(dead_code)]
#[path = "./utils/sub_process.rs"] mod sub_process;
#[path = "./utils/docker_args.rs"] mod docker_args;

pub fn command() -> Command<'static> {
    Command::new("run")
        .about(r#"This command would mount the same volumes to the current container
When the current path is under `/project`, a new container would use the same path as the working directory
Otherwise, this would change to be `/project`"#)
        .arg(docker_args::set_proxy_arg())
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

pub fn execute(sub_matches: &ArgMatches) {
    sub_process::exec(
        "docker",
        [
            vec![
                "run",
                "-w",
                &docker_args::get_working_directory(),
            ],
            get_volumes_from_args("/etc/hostname")
                .iter()
                .map(AsRef::as_ref)
                .collect(),
            sub_matches
                .values_of("args")
                .unwrap()
                .collect(),
        ]
            .concat()
            .as_slice(),
    );
}
