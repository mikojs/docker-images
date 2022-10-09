use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::psql::utils::{args, Database, Error};

pub fn command() -> Command<'static> {
    Command::new("table")
        .about("Reset a table in the database")
        .arg(
            Arg::new("table-name")
                .help("This table would be reset")
                .required(true),
        )
        .arg(
            Arg::new("delete")
                .help(
                    r#"This command would use `TRUNCATE` by default
If you want to use `DELETE`, you could use this option"#,
                )
                .long("delete")
                .action(ArgAction::SetTrue),
        )
}

pub fn execute(matches: &ArgMatches, db: Database) -> Result<(), Error> {
    let command = match args::get_one::<bool>(matches, "delete", &false) {
        true => "DELETE FROM",
        false => "TRUNCATE TABLE",
    };

    db.run(vec![
        "psql",
        &db.url,
        "-c",
        &format!("{} {};", command, args::value_of(matches, "table-name"),),
    ])?;
    Ok(())
}
