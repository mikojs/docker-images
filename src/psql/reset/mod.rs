use std::io::Error;

use clap::{Command, ArgMatches};

use crate::psql::utils::Database;

mod table;
mod sequence;

pub fn command() -> Command<'static> {
    Command::new("reset")
        .about("Reset something in the database")
        .subcommand_required(true)
        .subcommand(table::command())
        .subcommand(sequence::command())
}

pub fn execute(matches: &ArgMatches, db: Database) -> Result<(), Error> {
    match matches.subcommand() {
        Some(("table", sub_matches)) => table::execute(sub_matches, db)?,
        Some(("sequence", sub_matches)) => sequence::execute(sub_matches, db)?,
        _ => unreachable!(),
    };
    Ok(())
}
