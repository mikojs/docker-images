use clap::{Command, Arg, ArgMatches};

#[allow(dead_code)]
#[path = "../utils/args.rs"] mod args;
#[allow(dead_code)]
#[path = "../run.rs"] mod run;

pub fn command() -> Command<'static> {
    Command::new("clone")
        .about("Clone the database from the database url")
        .arg(
            Arg::new("file-name")
                .required(true)
        )
}

pub fn execute(matches: &ArgMatches, db_url: &str) {
    run::execute(
        &args::generate_arg_matches(
            vec![
                "-it",
                "--rm",
                "postgres:alpine",
                "pg_dump",
                "-Fc",
                "-f",
                matches
                    .value_of("file-name")
                    .unwrap(),
                db_url,
            ],
        ),
    );
}
