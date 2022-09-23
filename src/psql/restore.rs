use clap::{Command, Arg, ArgMatches};

#[allow(dead_code)]
#[path = "../utils/args.rs"] mod args;
#[allow(dead_code)]
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

pub fn execute(matches: &ArgMatches, db_name: &str, db_url: &str) {
    utils::check_db_url(db_name, db_url);
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
