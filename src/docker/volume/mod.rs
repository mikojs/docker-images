use clap::{ArgMatches, Command};

use crate::utils::Error;

mod backup;
mod reset;
mod restore;

pub fn command() -> Command<'static> {
    Command::new("volume")
        .about("Docker volume command")
        .subcommand(reset::command())
        .subcommand(backup::command())
        .subcommand(restore::command())
        .subcommand_required(true)
}

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    match matches.subcommand() {
        Some(("reset", sub_matches)) => reset::execute(sub_matches)?,
        Some(("backup", sub_matches)) => backup::execute(sub_matches)?,
        Some(("restore", sub_matches)) => restore::execute(sub_matches)?,
        _ => unreachable!(),
    };
    Ok(())
}
