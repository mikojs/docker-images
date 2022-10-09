use std::env;

use clap::{App, ArgMatches, Command};
use regex::Regex;

use utils::{args, Database, Error};

mod dump;
mod reset;
mod restore;
mod show;
pub mod utils;

pub fn get_db_names() -> Result<Vec<String>, Error> {
    let db_regex = Regex::new(r"_DB_URL$")?;
    let mut db_names = vec![];

    for (key, _) in env::vars() {
        if db_regex.is_match(&key) {
            db_names.push(key.replace("_DB_URL", "").replace("_", "-").to_lowercase());
        }
    }

    Ok(db_names)
}

pub fn command(app: App<'static>) -> Command<'static> {
    app.subcommand(show::command())
        .subcommand(dump::command())
        .subcommand(restore::command())
        .subcommand(reset::command())
        .arg(args::set_proxy(false))
}

pub fn execute(matches: &ArgMatches, db_name: &str) -> Result<(), Error> {
    let db = Database::new(db_name.to_string())?;

    match matches.subcommand() {
        Some(("show", _)) => show::execute(db),
        Some(("dump", sub_matches)) => dump::execute(sub_matches, db)?,
        Some(("restore", sub_matches)) => restore::execute(sub_matches, db)?,
        Some(("reset", sub_matches)) => reset::execute(sub_matches, db)?,
        _ => db.run([vec!["psql", &db.url], args::get_values_from_proxy(matches)].concat())?,
    };
    Ok(())
}
