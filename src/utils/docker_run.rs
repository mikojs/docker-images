use std::fs;
use std::env;
use std::process;

use regex::Regex;

use crate::utils::sub_process;
use crate::utils::get_container_name;
use crate::utils::get_working_dir;

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

fn get_env_file(container_name: &str) -> String {
    let file_path = "/root/.ddocker.env";
    let content = sub_process::exec_result(
        "docker",
        vec![
            "inspect",
            container_name,
            "--format",
            "{{.Config.Env}}",
        ],
    )
        .replace("[", "")
        .replace("]", "");
    let contents: Vec<&str> = content
        .split(" ")
        .filter(|x| !Regex::new(r"^PATH=.+").unwrap().is_match(x))
        .collect();
    let new_content = contents.join("\n");

    match fs::write(file_path, new_content) {
        Ok(_) => file_path.to_string(),
        _ => unreachable!(),
    }
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

pub fn main(args: Vec<&str>) {
    let container_name = get_container_name::main();

    sub_process::exec(
        "docker",
        [
            vec![
                "run",
                "-w",
                &get_working_dir::main(),
                "--env-file",
                &get_env_file(&container_name),
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
