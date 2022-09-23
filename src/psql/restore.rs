use clap::{Command, Arg, ArgMatches};

#[path = "../utils/args.rs"] mod args;
#[path = "../run.rs"] mod run;

#[path = "./utils.rs"] mod utils;

pub fn command() -> Command<'static> {
    Command::new("restore")
        .about("Restore the database from the cloned database file")
        .arg(
            Arg::new("file-name")
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
