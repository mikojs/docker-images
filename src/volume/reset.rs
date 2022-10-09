use clap::{Command, Arg, ArgMatches};

use crate::utils::{Error, ErrorKind, sub_process};

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
    let removed_result = sub_process::exec_result(
        "docker",
        vec!["volume", "rm", volume_name],
    )?
        .replace("\n", "");

    if removed_result != volume_name {
        return Err(
            Error::new(
                ErrorKind::DockerVolumeNotExpected,
                format!("Volume {} is removed, but the expected volume is {}", removed_result, volume_name),
            ),
        );
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
