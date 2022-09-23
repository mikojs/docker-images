use clap::{Command, Arg, ArgMatches};

#[allow(dead_code)]
#[path = "../../utils/args.rs"] mod args;
#[allow(dead_code)]
#[path = "../../run.rs"] mod run;

#[path = "../utils.rs"] mod utils;

pub fn command() -> Command<'static> {
    Command::new("sequence")
        .about("Reset a sequence in the database")
        .arg(
            Arg::new("sequence-name")
                .required(true)
        )
}

pub fn execute(matches: &ArgMatches, db_name: &str, db_url: &str) {
    utils::check_db_url(db_name, db_url);
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
                    "ALTER SEQUENCE {} RESTART WITH 1;",
                    matches
                        .value_of("sequence-name")
                        .unwrap(),
                ),
            ],
        ),
    );
}
