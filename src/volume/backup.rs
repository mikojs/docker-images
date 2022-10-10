use std::env;

use clap::{ArgMatches, Command};

use crate::utils::{args, docker, Error};

fn build_volume_args(matches: &ArgMatches) -> Vec<String> {
    let mut args = vec![];

    for volume_name in args::get_values_from_proxy(matches) {
        let volume_arg = format!("{0}:/backup/{0}", volume_name);

        args.push("-v".to_string());
        args.push(volume_arg);
    }

    args
}

pub fn command() -> Command<'static> {
    Command::new("backup")
        .about("Backup the docker volumes")
        .arg(
            args::set_proxy(true)
                .value_name("volume-names")
                .help("Those docker volumes would be backup"),
        )
}

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    docker::run(
        [
            build_volume_args(matches)
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
                    "tar -cvf backup.tar backup && docker cp backup.tar {0}:{1}",
                    docker::name()?,
                    env::current_dir()?.display().to_string(),
                ),
            ],
        ]
        .concat(),
    )?;
    Ok(())
}
