use std::process;
use std::io::Error;

use clap::{Command, Arg, ArgMatches};

use crate::utils::sub_process;

pub fn command() -> Command<'static> {
    Command::new("reset")
        .about("Reset a docker volume")
        .arg(
            Arg::new("volume-name")
                .help("This docker volume would be reset")
                .required(true)
        )
}

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    let volume_name = matches
        .value_of("volume-name")
        .unwrap();
    let remove_result = sub_process::exec_result(
        "docker",
        vec!["volume", "rm", volume_name],
    )?
        .replace("\n", "");

    if remove_result != volume_name {
        process::exit(1);
    }

    println!(
        "Reset `{}` volume",
        sub_process::exec_result(
            "docker",
            vec!["volume", "create", volume_name],
        )?
            .replace("\n", ""),
    );
    Ok(())
}
