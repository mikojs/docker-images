use std::env;

use clap::{Arg, ArgMatches, Command};

use crate::utils::{args, docker, Error};

pub fn command() -> Command<'static> {
    Command::new("backup").about("Backup a docker volume").arg(
        Arg::new("volume-name")
            .help("This docker volume would be backup")
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
            "tar -cvf {0}.tar {0} && docker cp {0}.tar {1}:{2}",
            volume_name,
            docker::name()?,
            env::current_dir()?.display().to_string(),
        ),
    ])?;
    Ok(())
}
