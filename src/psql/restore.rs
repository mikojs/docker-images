use std::process;

use clap::{Command, Arg, ArgMatches};
use regex::Regex;

use crate::psql::utils::{proxy_args, docker, Database};

pub fn command() -> Command<'static> {
    Command::new("restore")
        .about("Restore the database from a file")
        .arg(
            Arg::new("file-name")
                .required(true)
        )
        .arg(proxy_args::set_proxy_args(false))
}

pub fn execute(matches: &ArgMatches, mut db: Database) {
    let file_name = matches
        .value_of("file-name")
        .unwrap();
    let is_csv = Regex::new(r"\.csv$")
        .unwrap()
        .is_match(file_name);
    let db_url = db.url(true);
    let args = proxy_args::get_values_from_proxy_args(matches);

    if is_csv {
        if args.len() != 1 {
            eprint!("If you want to restore data from a CSV file, only one argument about SQL query could be accepted");
            process::exit(1);
        }

        docker::run(
            vec![
                "psql",
                db_url,
                "-c",
                &format!("\\copy {} FROM '{}' WITH csv", args[0], file_name),
            ],
        );
        return;
    }

    docker::run(
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
