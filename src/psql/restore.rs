use std::process;

use clap::{Command, Arg, ArgMatches};
use regex::Regex;

#[path = "../utils/proxy_args.rs"] mod proxy_args;

#[path = "./utils/docker_run.rs"] mod docker_run;

pub fn command() -> Command<'static> {
    Command::new("restore")
        .about("Restore the database from a file")
        .arg(
            Arg::new("file-name")
                .required(true)
        )
        .arg(proxy_args::set_proxy_args(false))
}

pub fn execute(matches: &ArgMatches, db_url: &str) {
    let file_name = matches
        .value_of("file-name")
        .unwrap();
    let is_csv = Regex::new(r"\.csv$")
        .unwrap()
        .is_match(file_name);
    let args = proxy_args::get_values_from_proxy_args(matches);

    if is_csv {
        if args.len() != 1 {
            eprint!("If you want to restore data from a CSV file, only one argument about SQL query could be accepted");
            process::exit(1);
        }

        docker_run::main(
            vec![
                "psql",
                "-c",
                &format!("\\copy {} FROM '{}' WITH csv", args[0], file_name),
            ],
        );
        return;
    }

    docker_run::main(
        [
            vec![
                "pg_restore",
                "--no-owner",
                "-x",
                "-d",
                db_url,
                file_name,
            ],
            args,
        ]
            .concat(),
    );
}
