use clap::{Command, Arg, ArgMatches};

#[allow(dead_code)]
#[path = "../../utils/args.rs"] mod args;
#[allow(dead_code)]
#[path = "../../run.rs"] mod run;

#[path = "../utils.rs"] mod utils;

pub fn command() -> Command<'static> {
    Command::new("table")
        .about("Reset a table in the database")
        .arg(
            Arg::new("table-name")
                .required(true)
        )
}

pub fn execute(matches: &ArgMatches, db_url: &str) {
    utils::check_db_url(db_url);
    run::execute(
        &args::generate_arg_matches(
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
