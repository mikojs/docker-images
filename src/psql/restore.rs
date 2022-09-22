use clap::{Command, Arg};

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

pub fn execute(db_url: &str) {
    /*
    run::execute(
        &args::generate_arg_matches(
            vec![
                "-it",
                "--rm",
                "postgres:alpine",
                "pg_restore",
                &db_url,
                "-f",
            ],
        ),
    );
    */
}
