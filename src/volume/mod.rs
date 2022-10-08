use std::io::Error;

use clap::{Command, ArgMatches};

mod reset;

pub fn command() -> Command<'static> {
    Command::new("volume")
        .about("Docker volume command")
        .subcommand(reset::command())
        .subcommand_required(true)
}

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    match matches.subcommand() {
        Some(("reset", sub_matches)) => Ok(reset::execute(sub_matches)?),
        _ => unreachable!(),
    }
}
