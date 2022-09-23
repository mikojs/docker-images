use clap::{Command, Arg, ArgMatches};

#[path = "../../utils/generate_arg_matches.rs"] mod generate_arg_matches;
#[allow(dead_code)]
#[path = "../../run.rs"] mod run;

#[path = "../utils/check_db_url.rs"] mod check_db_url;

pub fn command() -> Command<'static> {
    Command::new("table")
        .about("Reset a table in the database")
        .arg(
            Arg::new("table-name")
                .required(true)
        )
}

pub fn execute(matches: &ArgMatches, db_url: &str) {
    check_db_url::main(db_url);
    run::execute(
        &generate_arg_matches::main(
            vec![
                "-it",
                "--rm",
                "postgres:alpine",
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
        ),
    );
}
