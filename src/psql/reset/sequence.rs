use clap::{Command, Arg, ArgMatches};

use crate::psql::utils::docker;

pub fn command() -> Command<'static> {
    Command::new("sequence")
        .about("Reset a sequence in the database")
        .arg(
            Arg::new("sequence-name")
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
                "ALTER SEQUENCE {} RESTART WITH 1;",
                matches
                    .value_of("sequence-name")
                    .unwrap(),
            ),
        ],
    );
}
