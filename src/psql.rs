use std::env;
use std::process;

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
    let db_env_name = format!("{}_DB_URL", db_name.to_uppercase());

    match env::var(&db_env_name) {
        Ok(db_url) => {
            run::execute(
                sub_matches,
                vec!["-it", "--rm", "postgres:alpine", "psql", &db_url],
            );
        },
        _ => {
            eprint!("`{}` isn't in the environment variables.", db_env_name);
            process::exit(1);
        },
    }
}
