use std::env;

use clap::{Command, ArgMatches};

#[allow(dead_code)]
#[path = "./utils/args.rs"] mod args;
#[allow(dead_code)]
#[path = "./run.rs"] mod run;

pub fn command(db_name: &str) -> Command<'static> {
    Command::new(db_name)
        .about("Specific db")
        .arg(args::set_proxy_arg(false))
}

pub fn execute(sub_matches: &ArgMatches, db_name: &str) {
    let db_url = env::var(
        format!("{}_DB_URL", db_name.to_uppercase())
    )
        .expect("Couldn't get the db url");

    run::execute(
        sub_matches,
        vec!["-it", "--rm", "postgres:alpine", "psql", &db_url],
    );
}
