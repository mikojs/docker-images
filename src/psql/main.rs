use std::env;
use std::process;

use clap::{App, Command, ArgMatches};
use regex::Regex;

#[path = "../utils/proxy_args.rs"] mod proxy_args;
#[path = "../utils/get_version.rs"] mod get_version;
#[path = "../utils/docker_run.rs"] mod docker_run;

#[path = "./dump.rs"] mod dump;
#[path = "./restore.rs"] mod restore;
#[path = "./reset/main.rs"] mod reset;

fn get_db_url(db_name: &str) -> String {
    let db_env_name = format!(
        "{}_DB_URL",
        db_name
            .replace("-", "_")
            .to_uppercase(),
    );

    if let Ok(db_url) = env::var(&db_env_name) {
        return db_url;
    }

    eprint!(
        "`{}` isn't in the environment variables.",
        db_env_name,
    );
    process::exit(1);
}

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
        .subcommand(
            Command::new("show")
                .about("Show the database url")
        )
        .subcommand(dump::command())
        .subcommand(restore::command())
        .subcommand(reset::command())
        .arg(proxy_args::set_proxy_args(false))
}

pub fn execute(matches: &ArgMatches, db_name: &str) {
    let db_url = get_db_url(db_name);

    match matches.subcommand() {
        Some(("show", _)) => println!("{}", db_url),
        Some(("dump", sub_matches)) => dump::execute(sub_matches, db_name, &db_url),
        Some(("restore", sub_matches)) => restore::execute(sub_matches, db_name, &db_url),
        Some(("reset", sub_matches)) => reset::execute(sub_matches, db_name, &db_url),
        _ => docker_run::main(
            [
                vec![
                    "-it",
                    "--rm",
                    &get_version::main("postgres", "postgres", vec!["alpine"]),
                    "psql",
                    &db_url,
                ],
                proxy_args::get_values_from_proxy_args(matches),
            ]
                .concat(),
        ),
    }
}
