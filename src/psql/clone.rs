use clap::{Command, Arg, ArgMatches};

#[path = "../utils/args.rs"] mod args;
#[path = "../utils/generate_arg_matches.rs"] mod generate_arg_matches;
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

pub fn execute(matches: &ArgMatches, db_url: &str) {
    utils::check_db_url(db_url);
    run::execute(
        &generate_arg_matches::main(
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
