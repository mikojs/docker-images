use clap::{Command, Arg, ArgMatches};

use crate::utils::sub_process;

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

    println!(
        "Remove `{}` volume",
        sub_process::exec_result("docker", vec!["volume", "rm", volume_name])
            .replace("\n", ""),
    );
    println!(
        "Create `{}` volume",
        sub_process::exec_result("docker", vec!["volume", "create", volume_name])
            .replace("\n", ""),
    );
}
