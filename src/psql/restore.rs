use clap::{Command, Arg, ArgMatches};

#[path = "../utils/args.rs"] mod args;
#[path = "../utils/generate_arg_matches.rs"] mod generate_arg_matches;
#[path = "../run.rs"] mod run;

#[path = "./utils/check_db_url.rs"] mod check_db_url;

pub fn command() -> Command<'static> {
    Command::new("restore")
        .about("Restore the database from the cloned database file")
        .arg(
            Arg::new("file-name")
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
                "pg_restore",
                "--no-owner",
                "-x",
                "-d",
                db_url,
                matches
                    .value_of("file-name")
                    .unwrap(),
            ],
        ),
    );
}
