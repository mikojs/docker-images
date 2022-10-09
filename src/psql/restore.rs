use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::psql::utils::{args, Database, Error};

pub fn command() -> Command<'static> {
    Command::new("restore")
        .about("Restore the database from a file")
        .arg(
            Arg::new("file-name")
                .help("The file name is used to restore the data")
                .required(true),
        )
        .arg(
            Arg::new("format")
                .help("Use SQL to format the data when restoring the data from the CSV file")
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
                    &format!("\\copy {} FROM '{}' WITH csv", format, file_name),
                ],
                args,
            ]
            .concat(),
        )?;
        return Ok(());
    }

    db.run(
        [
            vec!["pg_restore", "--no-owner", "-x", "-d", &db.url, file_name],
            args,
        ]
        .concat(),
    )?;
    Ok(())
}
