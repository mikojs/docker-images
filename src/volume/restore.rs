use std::env;

use clap::{Arg, ArgMatches, Command};

use crate::utils::{args, docker, Error};

pub fn command() -> Command<'static> {
    Command::new("restore")
        .about("Restore a docker volume")
        .arg(
            Arg::new("volume-name")
                .help("Restore a docker volume from a tar file")
                .required(true),
        )
}

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    let volume_name = args::value_of(matches, "volume-name");

    docker::run(vec![
        "-it",
        "--rm",
        "-v",
        &format!("{0}:/{0}", volume_name),
        "-w",
        "/",
        "docker",
        "/bin/sh",
        "-c",
        &format!(
            "docker cp {0}:{1}/{2}.tar . && tar -xvf {2}.tar",
            docker::name()?,
            env::current_dir()?.display().to_string(),
            volume_name,
        ),
    ])?;
    Ok(())
}
