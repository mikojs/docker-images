use clap::{Command, Arg, ArgMatches};

use crate::utils::{docker};

pub fn command() -> Command<'static> {
    Command::new("reset")
        .about("Reset a docker volume")
        .arg(
            Arg::new("volume-name")
                .required(true)
        )
}

pub fn execute(matches: &ArgMatches) {
    let volume_name = matches
        .value_of("volume-name")
        .unwrap();

    docker::run(vec!["volume", "rm", volume_name]);
    docker::run(vec!["volume", "create", volume_name]);
}
