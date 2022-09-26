use std::env;
use std::process;

use clap::{App, Command, ArgMatches};
use regex::Regex;

#[path = "../utils/proxy_args.rs"] mod proxy_args;

#[path = "./dump.rs"] mod dump;
#[path = "./restore.rs"] mod restore;
#[path = "./reset/main.rs"] mod reset;
#[path = "./utils/docker_run.rs"] mod docker_run;
#[path = "./utils/check_db_url.rs"] mod check_db_url;

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
        Some(("dump", sub_matches)) => {
            check_db_url::main(db_name, &db_url, true);
            dump::execute(sub_matches, &db_url);
        },
        Some(("restore", sub_matches)) => {
            check_db_url::main(db_name, &db_url, false);
            restore::execute(sub_matches, &db_url);
        },
        Some(("reset", sub_matches)) => {
            check_db_url::main(db_name, &db_url, false);
            reset::execute(sub_matches, &db_url);
        },
        _ => {
            check_db_url::main(db_name, &db_url, true);
            docker_run::main(
                [
                    vec!["psql", &db_url],
                    proxy_args::get_values_from_proxy_args(matches),
                ]
                    .concat(),
            );
        },
    }
}