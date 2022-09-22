use clap::{Command, Arg, ArgMatches};

#[allow(dead_code)]
#[path = "./utils/args.rs"] mod args;

#[allow(dead_code)]
#[path = "./run.rs"] mod run;
#[allow(dead_code)]
#[path = "./psql_show.rs"] mod psql_show;

pub fn command() -> Command<'static> {
    Command::new("clone")
        .about("Clone the database from the database url")
}

pub fn execute(sub_matches: &ArgMatches, db_name: &str) {
    run::execute(
        &Command::new("clone")
            .arg(args::set_proxy_arg(true))
            .get_matches_from(vec![
                "clone",
                "-it",
                "--rm",
                "postgres:alpine",
                "pg_dump",
                &psql_show::execute(db_name),
                "-Fc",
            ]),
    );
}
