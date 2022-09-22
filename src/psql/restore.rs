use clap::{Command, Arg, ArgMatches};

#[allow(dead_code)]
#[path = "../utils/args.rs"] mod args;
#[allow(dead_code)]
#[path = "../run.rs"] mod run;

pub fn command() -> Command<'static> {
    Command::new("restore")
        .about("Restore the database from the cloned database file")
        .arg(
            Arg::new("file-name")
                .required(true)
        )
}

pub fn execute(sub_matches: &ArgMatches, db_url: &str) {
    // TODO: should check db_url if it's included in the danger db urls
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
                sub_matches
                    .value_of("file-name")
                    .unwrap(),
            ],
        ),
    );
}
