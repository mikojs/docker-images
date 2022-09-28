use std::fs;
use std::env;
use std::process;
use std::path::Path;

use regex::Regex;

use crate::utils::sub_process;

mod env_file;

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
        .replace("\n", "")
}

fn filter_args(args: Vec<&str>) -> Vec<&str> {
    if args[1].is_empty() {
        return vec![];
    }

    args
}

fn get_version(versions: Vec<&str>) -> String {
    let env_name_regex = Regex::new(r"DOCKER_.+_VERSION")
        .unwrap();

    for version in versions {
        if env_name_regex.is_match(version) {
            if let Ok(env) = env::var(version) {
                return env;
            }
        } else if !version.is_empty() {
            return version.to_string();
        }
    }

    "alpine".to_string()
}

fn transform_image_version(arg: &str) -> String {
    let is_specific_image_version = Regex::new(r".+:<.+>")
        .unwrap()
        .is_match(arg);

    if !is_specific_image_version {
        return arg.to_string();
    }

    let data: Vec<&str> = arg.split(":")
        .collect();

    if data.len() != 2 {
        eprintln!("Couldn't parse {}", arg);
        process::exit(1);
    }

    let versions_str = data[1]
        .replace("<", "")
        .replace(">", "");
    let versions: Vec<&str> = versions_str
        .split("|")
        .collect();

    if versions.len() == 0 {
        eprintln!("Couldn't parse {}", arg);
        process::exit(1);
    }

    let default_version = versions[versions.len() - 1];
    let version = get_version(versions);
    let image = format!("{}:{}", data[0], version);

    if version != default_version {
        println!("Custom Image: `{}`", image);
    }

    image
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
                &env_file::get(&container_name),
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
                .map(|&x| transform_image_version(x))
                .collect::<Vec<String>>()
                .iter()
                .map(AsRef::as_ref)
                .collect(),
        ]
            .concat(),
    );
}
