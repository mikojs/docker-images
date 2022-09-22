use clap::{Command, Arg, ArgMatches};

pub fn command() -> Command<'static> {
    Command::new("reset")
        .about("Reset something in the database")
        .subcommand_required(true)
}

pub fn execute(matches: &ArgMatches, db_url: &str) {
    match matches.subcommand() {
        _ => unreachable!(),
    }
}
