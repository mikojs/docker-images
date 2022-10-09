use std::env;
use std::fs;
use std::path::Path;

use regex::Regex;

use crate::utils::{sub_process, Error};

mod env_file;
mod image;

const HOSTNAME_PATH: &str = "/etc/hostname";

pub fn name() -> Result<String, Error> {
    if !Path::new(HOSTNAME_PATH).exists() {
        return Ok("".to_string());
    }

    Ok(fs::read_to_string(HOSTNAME_PATH)?.replace("\n", ""))
}

pub fn working_dir() -> Result<String, Error> {
    let cwd = env::current_dir()?.display().to_string();
    let is_work = Regex::new(r"^/root/work")?.is_match(&cwd);

    if is_work {
        return Ok(cwd);
    }

    Ok("/root/work".to_string())
}

fn get_network_name(container_name: &str) -> Result<String, Error> {
    Ok(sub_process::exec_result(
        "docker",
        vec![
            "inspect",
            container_name,
            "--format",
            "{{.HostConfig.NetworkMode}}",
        ],
    )?
    .replace("\n", ""))
}

fn filter_args(args: Vec<&str>) -> Vec<&str> {
    if args[1].is_empty() {
        return vec![];
    }

    args
}

pub fn run(args: Vec<&str>) -> Result<(), Error> {
    let container_name = name()?;
    let mut new_args = vec![];

    for arg in &args {
        let is_specific_image_version = Regex::new(image::NAME_PATTERN)?.is_match(arg);

        if is_specific_image_version {
            new_args.push(image::name(arg)?);
        } else {
            new_args.push(arg.to_string());
        }
    }

    sub_process::exec(
        "docker",
        [
            vec![
                "run",
                "-w",
                &working_dir()?,
                "--env-file",
                &env_file::get(&container_name)?,
            ],
            filter_args(vec!["--volumes-from", &container_name]),
            filter_args(vec!["--network", &get_network_name(&container_name)?]),
            new_args.iter().map(AsRef::as_ref).collect(),
        ]
        .concat(),
    )?;
    Ok(())
}
