use clap::{Command, Arg, ArgMatches};

#[path = "../../utils/args.rs"] mod args;
#[path = "../../utils/generate_arg_matches.rs"] mod generate_arg_matches;
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

pub fn execute(matches: &ArgMatches, db_url: &str) {
    utils::check_db_url(db_url);
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
                    "ALTER SEQUENCE {} RESTART WITH 1;",
                    matches
                        .value_of("sequence-name")
                        .unwrap(),
                ),
            ],
        ),
    );
}
