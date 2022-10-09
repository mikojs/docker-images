use clap::{Command, Arg, ArgAction, ArgMatches};

use crate::psql::utils::{Error, proxy_args, Database};

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

pub fn execute(matches: &ArgMatches, db: Database) -> Result<(), Error> {
    let file_name = proxy_args::value_of(matches, "file-name");
    let default_format = "".to_string();
    let format = proxy_args::get_one::<String>(matches, "format", &default_format);
    let args = proxy_args::get_values_from_proxy_args(matches);

    if !format.is_empty() {
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
        )?;
        return Ok(());
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
    )?;
    Ok(())
}
