use clap::{Arg, ArgMatches, Command};

use crate::utils::{args, Error};

pub fn command() -> Command<'static> {
    Command::new("backup").about("Backup a docker volume").arg(
        Arg::new("volume-name")
            .help("This docker volume would be backup")
            .required(true),
    )
}

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    let volume_name = args::value_of(matches, "volume-name");

    println!("{}", volume_name);
    Ok(())
}
