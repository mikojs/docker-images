use clap::{Command, Arg, ArgMatches};

#[allow(dead_code)]
#[path = "../utils/args.rs"] mod args;
#[path = "../utils/generate_arg_matches.rs"] mod generate_arg_matches;
#[allow(dead_code)]
#[path = "../run.rs"] mod run;

#[path = "./utils/check_db_url.rs"] mod check_db_url;

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
    check_db_url::main(db_name, db_url, true);
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
