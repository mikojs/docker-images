use std::fs;
use std::env;
use std::path::Path;

use regex::Regex;

use crate::utils::sub_process;

mod env_file;
mod image;

const HOSTNAME_PATH: &str = "/etc/hostname";

pub fn name() -> String {
    if !Path::new(HOSTNAME_PATH).exists() {
        return "".to_string();
    }

    fs::read_to_string(HOSTNAME_PATH)
        .expect("Couldn't read the file")
        .replace("\n", "")
}

pub fn working_dir() -> String {
    let cwd = env::current_dir()
        .expect("Couldn't get the currenct directory")
        .display()
        .to_string();
    let is_work = Regex::new(r"^/root/work")
        .unwrap()
        .is_match(&cwd);

    if is_work {
        return cwd;
    }

    "/root/work".to_string()
}

fn get_network_name(container_name: &str) -> String {
    sub_process::exec_result(
        "docker",
        vec![
            "inspect",
            container_name,
            "--format",
            "{{.HostConfig.NetworkMode}}",
        ],
    )
        .expect("TODO")
        .replace("\n", "")
}

fn filter_args(args: Vec<&str>) -> Vec<&str> {
    if args[1].is_empty() {
        return vec![];
    }

    args
}
fn transform_image_name(arg: &str) -> String {
    let is_specific_image_version = Regex::new(image::NAME_PATTERN)
        .unwrap()
        .is_match(arg);

    if !is_specific_image_version {
        return arg.to_string();
    }

    image::name(arg)
}

pub fn run(args: Vec<&str>) {
    let container_name = name();

    sub_process::exec(
        "docker",
        [
            vec![
                "run",
                "-w",
                &working_dir(),
                "--env-file",
                &env_file::get(&container_name)
                    .expect("TODO"),
            ],
            filter_args(
                vec!["--volumes-from", &container_name],
            ),
            filter_args(
                vec![
                    "--network",
                    &get_network_name(&container_name),
                ],
            ),
            args
                .iter()
                .map(|&x| transform_image_name(x))
                .collect::<Vec<String>>()
                .iter()
                .map(AsRef::as_ref)
                .collect(),
        ]
            .concat(),
    );
}
