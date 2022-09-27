use clap::{Command, Arg, ArgMatches};

use crate::psql::utils::docker;

pub fn command() -> Command<'static> {
    Command::new("table")
        .about("Reset a table in the database")
        .arg(
            Arg::new("table-name")
                .required(true)
        )
}

pub fn execute(matches: &ArgMatches, db_url: &str) {
    docker::run(
        vec![
            "psql",
            db_url,
            "-c",
            &format!(
                "TRUNCATE TABLE {};",
                matches
                    .value_of("table-name")
                    .unwrap(),
            ),
        ],
    );
}
