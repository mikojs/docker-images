use clap::{Command, Arg, ArgMatches};

#[path = "../utils/docker_run.rs"] mod docker_run;

pub fn command() -> Command<'static> {
    Command::new("table")
        .about("Reset a table in the database")
        .arg(
            Arg::new("table-name")
                .required(true)
        )
}

pub fn execute(matches: &ArgMatches, db_name: &str, db_url: &str) {
    docker_run::main(
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
