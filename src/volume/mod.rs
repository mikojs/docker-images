use clap::{ArgMatches, Command};

use crate::utils::Error;

mod reset;

pub fn command() -> Command<'static> {
    Command::new("volume")
        .about("Docker volume command")
        .subcommand(reset::command())
        .subcommand_required(true)
}

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    match matches.subcommand() {
        Some(("reset", sub_matches)) => reset::execute(sub_matches)?,
        _ => unreachable!(),
    };
    Ok(())
}
