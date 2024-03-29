use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::psql::utils::{args, Database, Error};

pub fn command() -> Command<'static> {
    Command::new("dump")
        .about("Dump the database to a file")
        .arg(
            Arg::new("file-name")
                .help("Dump the data to this file")
                .required(true),
        )
        .arg(
            Arg::new("format")
                .help("Use SQL to format the data when dumping the CSV file")
                .long("format")
                .action(ArgAction::Set),
        )
        .arg(args::set_proxy(false))
}

pub fn execute(matches: &ArgMatches, db: Database) -> Result<(), Error> {
    let file_name = args::value_of(matches, "file-name");
    let default_format = "".to_string();
    let format = args::get_one::<String>(matches, "format", &default_format);
    let args = args::get_values_from_proxy(matches);

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

    db.run([vec!["pg_dump", "-Fc", "-f", file_name, &db.url], args].concat())?;
    Ok(())
}
