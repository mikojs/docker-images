use clap::{Command, Arg, ArgAction, ArgMatches};

use crate::psql::utils::{Error, proxy_args, Database};

pub fn command() -> Command<'static> {
    Command::new("table")
        .about("Reset a table in the database")
        .arg(
            Arg::new("table-name")
                .help("This table would be reset")
                .required(true)
        )
        .arg(
            Arg::new("delete")
                .help(r#"This command would use `TRUNCATE` by default
If you want to use `DELETE`, you could use this option"#)
                .long("delete")
                .action(ArgAction::SetTrue)
        )
}

pub fn execute(matches: &ArgMatches, db: Database) -> Result<(), Error> {
    let command = match matches.get_one::<bool>("delete") {
        Some(true) => "DELETE FROM",
        _ => "TRUNCATE TABLE"
    };

    db.run(
        vec![
            "psql",
            &db.url,
            "-c",
            &format!(
                "{} {};",
                command,
                proxy_args::value_of(matches, "table-name"),
            ),
        ],
    )?;
    Ok(())
}
