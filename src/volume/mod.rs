use clap::{Command, ArgMatches};

pub fn command() -> Command<'static> {
    Command::new("volume")
        .about("Docker volume command")
}

pub fn execute(matches: &ArgMatches, db) {
    match matches.subcommand() {
        _ => unreachable!(),
    }
}
