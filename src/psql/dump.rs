use clap::{Command, Arg, ArgAction, ArgMatches};

use crate::psql::utils::{proxy_args, Database};

pub fn command() -> Command<'static> {
    Command::new("dump")
        .about("Dump the database to a file")
        .arg(
            Arg::new("file-name")
                .help("Dump the data to this file")
                .required(true)
        )
        .arg(
            Arg::new("format")
                .help("Use SQL to format the data when dumping the CSV file")
                .long("format")
                .action(ArgAction::Set)
        )
        .arg(proxy_args::set_proxy_args(false))
}

pub fn execute(matches: &ArgMatches, db: Database) {
    let file_name = matches
        .value_of("file-name")
        .unwrap();
    let args = proxy_args::get_values_from_proxy_args(matches);

    if let Some(format) = matches.get_one::<String>("format") {
        db.run(
            [
                vec![
                    "psql",
                    &db.url,
                    "-c",
                    &format!("\\copy ({}) TO '{}' WITH csv", format, file_name),
                ],
                args,
            ]
                .concat(),
        );
        return;
    }

    db.run(
        [
            vec![
                "pg_dump",
                "-Fc",
                "-f",
                file_name,
                &db.url,
            ],
            args,
        ]
            .concat(),
    );
}
