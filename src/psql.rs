use std::env;
use std::process;

use clap::{App, Command, ArgMatches};

#[allow(dead_code)]
#[path = "./utils/args.rs"] mod args;

#[allow(dead_code)]
#[path = "./run.rs"] mod run;
#[path = "./psql_clone.rs"] mod psql_clone;

fn get_db_url(db_name: &str) -> String {
    let db_env_name = format!("{}_DB_URL", db_name.to_uppercase());

    if let Ok(db_url) = env::var(&db_env_name) {
        return db_url;
    }

    eprint!(
        "`{}` isn't in the environment variables.",
        db_env_name,
    );
    process::exit(1);
}

pub fn command(app: App<'static>) -> Command<'static> {
    app
        .subcommand(
            Command::new("show")
                .about("Show the database url")
        )
        .subcommand(psql_clone::command())
        .arg(args::set_proxy_arg(false))
}

pub fn execute(sub_matches: &ArgMatches, db_name: &str) {
    let db_url = get_db_url(db_name);

    match sub_matches.subcommand() {
        Some(("show", _)) => println!("{}", db_url),
        Some(("clone", _)) => psql_clone::execute(&db_url),
        _ => run::execute(
            &args::generate_arg_matches(
                [
                    vec![
                        "-it",
                        "--rm",
                        "postgres:alpine",
                        "psql",
                        &db_url,
                    ],
                    args::get_values_from_args(sub_matches),
                ]
                    .concat(),
            ),
        ),
    }
}
