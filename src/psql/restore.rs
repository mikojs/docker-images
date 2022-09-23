use clap::{Command, Arg, ArgMatches};

#[allow(dead_code)]
#[path = "../utils/args.rs"] mod args;
#[path = "../utils/generate_arg_matches.rs"] mod generate_arg_matches;
#[path = "../utils/get_version.rs"] mod get_version;
#[allow(dead_code)]
#[path = "../run.rs"] mod run;

#[path = "./utils/check_db_url.rs"] mod check_db_url;

pub fn command() -> Command<'static> {
    Command::new("restore")
        .about("Restore the database from a file")
        .arg(
            Arg::new("file-name")
                .required(true)
        )
        .arg(args::set_proxy_arg(false))
}

pub fn execute(matches: &ArgMatches, db_name: &str, db_url: &str) {
    check_db_url::main(db_name, db_url, false);
    run::execute(
        &generate_arg_matches::main(
            [
                vec![
                    "-it",
                    "--rm",
                    &get_version::main("postgres", "postgres", vec!["alpine"]),
                    "pg_restore",
                    "--no-owner",
                    "-x",
                    "-d",
                    db_url,
                    matches
                        .value_of("file-name")
                        .unwrap(),
                ],
                args::get_values_from_args(matches),
            ]
                .concat(),
        ),
    );
}
