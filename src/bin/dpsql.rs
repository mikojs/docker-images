use std::env;

use clap::{crate_version, Command};
use regex::Regex;

#[path = "../psql/main.rs"] mod psql;

fn get_db_names() -> Vec<String> {
    let db_regex = Regex::new(r"DB_URL$")
        .unwrap();
    let mut db_names = vec![];

    for (key, _) in env::vars() {
        if db_regex.is_match(&key) {
            db_names.push(
                key
                    .replace("_DB_URL", "")
                    .to_lowercase(),
            );
        }
    }

    db_names
}

fn main() {
    let mut app = psql::command(
        Command::new("dpsql")
            .version(crate_version!())
            .about("Use psql command in the docker container")
    );

    for db_name in get_db_names() {
        app = app.subcommand(
            psql::command(
                Command::new(&db_name)
                    .about("Database")
            ),
        );
    }

    let matches = app.get_matches();

    if let Some((sub_command, sub_matches)) = matches.subcommand() {
        for db_name in get_db_names() {
            if sub_command == db_name {
                psql::execute(sub_matches, sub_command);
                return;
            }
        }
    }

    psql::execute(&matches, "default");
}
