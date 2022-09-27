use clap::{Command, Arg, ArgMatches};

use crate::psql::utils::{docker, Database};

pub fn command() -> Command<'static> {
    Command::new("table")
        .about("Reset a table in the database")
        .arg(
            Arg::new("table-name")
                .required(true)
        )
}

pub fn execute(matches: &ArgMatches, db: Database) {
    docker::run(
        vec![
            "psql",
            db.url(),
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
