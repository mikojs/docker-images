use clap::{Command, ArgMatches};

#[path = "./table.rs"] mod table;

pub fn command() -> Command<'static> {
    Command::new("reset")
        .about("Reset something in the database")
        .subcommand_required(true)
        .subcommand(table::command())
}

pub fn execute(matches: &ArgMatches, db_url: &str) {
    match matches.subcommand() {
        Some(("table", sub_matches)) => table::execute(sub_matches, db_url),
        _ => unreachable!(),
    }
}
