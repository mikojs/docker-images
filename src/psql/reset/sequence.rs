use clap::{Command, Arg, ArgMatches};

#[path = "../../utils/generate_arg_matches.rs"] mod generate_arg_matches;
#[allow(dead_code)]
#[path = "../../run.rs"] mod run;

#[path = "../utils/check_db_url.rs"] mod check_db_url;

pub fn command() -> Command<'static> {
    Command::new("sequence")
        .about("Reset a sequence in the database")
        .arg(
            Arg::new("sequence-name")
                .required(true)
        )
}

pub fn execute(matches: &ArgMatches, db_name: &str, db_url: &str) {
    check_db_url::main(db_name, db_url, false);
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
