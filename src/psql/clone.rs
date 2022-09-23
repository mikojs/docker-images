use clap::{Command, Arg, ArgMatches};

#[allow(dead_code)]
#[path = "../utils/args.rs"] mod args;
#[allow(dead_code)]
#[path = "../run.rs"] mod run;

#[path = "./utils.rs"] mod utils;

fn get_table_name(matches: &ArgMatches) -> String {
    if let Some(table_name) = matches.value_of("table-name") {
        return table_name.to_string();
    }

    "".to_string()
}

pub fn command() -> Command<'static> {
    Command::new("clone")
        .about("Clone the database from the database url")
        .arg(
            Arg::new("file-name")
                .required(true)
        )
        .arg(
            Arg::new("table-name")
                .short('t')
                .long("table")
                .help("Choose a table")
                .takes_value(true)
        )
}

pub fn execute(matches: &ArgMatches, db_name: &str, db_url: &str) {
    utils::check_db_url(db_name, db_url);
    run::execute(
        &args::generate_arg_matches(
            [
                vec![
                    "-it",
                    "--rm",
                    "postgres:alpine",
                    "pg_dump",
                    "-Fc",
                ],
                args::filter_args(
                    vec!["-t", &get_table_name(matches)],
                ),
                vec![
                    "-f",
                    matches
                        .value_of("file-name")
                        .unwrap(),
                    db_url,
                ],
            ]
                .concat(),
        ),
    );
}
