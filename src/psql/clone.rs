use clap::{Command};

#[allow(dead_code)]
#[path = "../utils/args.rs"] mod args;
#[allow(dead_code)]
#[path = "../run.rs"] mod run;

pub fn command() -> Command<'static> {
    Command::new("clone")
        .about("Clone the database from the database url")
}

pub fn execute(db_url: &str) {
    run::execute(
        &args::generate_arg_matches(
            vec![
                "-it",
                "--rm",
                "postgres:alpine",
                "pg_dump",
                db_url,
                "-Fc",
            ],
        ),
    );
}
