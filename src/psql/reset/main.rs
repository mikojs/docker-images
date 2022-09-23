use clap::{Command, ArgMatches};

#[path = "./table.rs"] mod table;
#[path = "./sequence.rs"] mod sequence;

pub fn command() -> Command<'static> {
    Command::new("reset")
        .about("Reset something in the database")
        .subcommand_required(true)
        .subcommand(table::command())
        .subcommand(sequence::command())
}

pub fn execute(matches: &ArgMatches, db_name: &str, db_url: &str) {
    match matches.subcommand() {
        Some(("table", sub_matches)) => table::execute(sub_matches, db_name, db_url),
        Some(("sequence", sub_matches)) => sequence::execute(sub_matches, db_name, db_url),
        _ => unreachable!(),
    }
}
