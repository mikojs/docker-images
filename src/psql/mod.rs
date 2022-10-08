use std::env;

use clap::{App, Command, ArgMatches};
use regex::Regex;

use utils::{proxy_args, Database};

mod show;
mod dump;
mod restore;
mod reset;
mod utils;

pub fn get_db_names() -> Vec<String> {
    let db_regex = Regex::new(r"_DB_URL$")
        .unwrap();
    let mut db_names = vec![];

    for (key, _) in env::vars() {
        if db_regex.is_match(&key) {
            db_names.push(
                key
                    .replace("_DB_URL", "")
                    .replace("_", "-")
                    .to_lowercase(),
            );
        }
    }

    db_names
}

pub fn command(app: App<'static>) -> Command<'static> {
    app
        .subcommand(show::command())
        .subcommand(dump::command())
        .subcommand(restore::command())
        .subcommand(reset::command())
        .arg(proxy_args::set_proxy_args(false))
}

pub fn execute(matches: &ArgMatches, db_name: &str) {
    let db = Database::new(db_name.to_string());

    match matches.subcommand() {
        Some(("show", _)) => show::execute(db),
        Some(("dump", sub_matches)) => dump::execute(sub_matches, db),
        Some(("restore", sub_matches)) => restore::execute(sub_matches, db),
        Some(("reset", sub_matches)) => reset::execute(sub_matches, db),
        _ => db.run(
            [
                vec!["psql", &db.url],
                proxy_args::get_values_from_proxy_args(matches),
            ]
                .concat(),
        ),
    }
}
