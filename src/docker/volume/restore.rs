use std::env;

use clap::{Arg, ArgMatches, Command};
use regex::Regex;

use crate::utils::{args, docker, sub_process, Error};

fn build_volume_args(tar_name: &str) -> Result<Vec<String>, Error> {
    let mut args: Vec<String> = vec![];
    let result = sub_process::exec_result("tar", vec!["-tf", tar_name])?;
    let slash_reg = Regex::new(r"/.*")?;

    for line in result.split("\n") {
        let file = line.replace("backup/", "");

        if file.is_empty() {
            continue;
        }

        let volume_name = format!("{0}:/backup/{0}", slash_reg.replace_all(&file, ""));

        if args
            .iter()
            .find(|&x| x.to_string() == volume_name)
            .is_none()
        {
            args.push("-v".to_string());
            args.push(volume_name);
        }
    }

    Ok(args)
}

pub fn command() -> Command<'static> {
    Command::new("restore")
        .about("Restore the docker volumes")
        .arg(
            Arg::new("tar-name")
                .help("Restore the docker volumes from the tar file")
                .required(true),
        )
}

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    let tar_name = args::value_of(matches, "tar-name");

    docker::run(
        [
            build_volume_args(tar_name)?
                .iter()
                .map(AsRef::as_ref)
                .collect(),
            vec![
                "-it",
                "--rm",
                "-w",
                "/",
                "docker",
                "/bin/sh",
                "-c",
                &format!(
                    "docker cp {0}:{1}/{2} . && tar -xvf {2}",
                    docker::name()?,
                    env::current_dir()?.display().to_string(),
                    tar_name,
                ),
            ],
        ]
        .concat(),
    )?;
    Ok(())
}
