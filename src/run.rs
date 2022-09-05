use std::fs;
use std::path::Path;

use clap::{Command, ArgMatches};

#[path = "./utils/sub_process.rs"] mod sub_process;
#[path = "./utils/args.rs"] mod args;

pub fn command() -> Command<'static> {
    Command::new("run")
        .about(r#"This command would mount the same volumes to the current container
When the current path is under `/project`, a new container would use the same path as the working directory
Otherwise, this would change to be `/project`"#)
        .arg(args::set_proxy_arg())
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

fn get_container_network_args() -> Vec<String> {
    let mut args: Vec<String> = []
        .to_vec();
    let network = sub_process::exec_result(
        "docker",
        &[
            "inspect",
            // container name
            "--format",
            "{{.HostConfig.NetworkMode}}",
        ],
    );

    if !network.is_empty() {
        args.push("--network".to_string());
        args.push(network.replace("\n", ""));
    }

    args
}

pub fn execute(sub_matches: &ArgMatches) {
    println!(">>>> {:?}", get_container_network_args());

    sub_process::exec(
        "docker",
        [
            vec![
                "run",
                "-w",
                &args::get_working_directory(),
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
