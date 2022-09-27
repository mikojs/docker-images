use std::process;

use clap::{Command, Arg, ArgMatches};
use regex::Regex;

use crate::utils::proxy_args;
use crate::psql::utils::docker;

pub fn command() -> Command<'static> {
    Command::new("dump")
        .about("Dump the database to a file")
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
            eprint!("If you want to dump data into a CSV file, only one argument about SQL query could be accepted");
            process::exit(1);
        }

        docker::run(
            vec![
                "psql",
                "-c",
                &format!("\\copy ({}) TO '{}' WITH csv", args[0], file_name),
            ],
        );
        return;
    }

    docker::run(
        [
            vec![
                "pg_dump",
                "-Fc",
                "-f",
                file_name,
                db_url,
            ],
            args,
        ]
            .concat(),
    );
}
