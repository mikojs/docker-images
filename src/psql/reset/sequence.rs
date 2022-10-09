use clap::{Command, Arg, ArgMatches};

use crate::psql::utils::{Error, proxy_args, Database};

pub fn command() -> Command<'static> {
    Command::new("sequence")
        .about("Reset a sequence in the database")
        .arg(
            Arg::new("sequence-name")
                .help("This sequence would be reset")
                .required(true)
        )
}

pub fn execute(matches: &ArgMatches, db: Database) -> Result<(), Error> {
    db.run(
        vec![
            "psql",
            &db.url,
            "-c",
            &format!(
                "ALTER SEQUENCE {} RESTART WITH 1;",
                proxy_args::value_of(matches, "sequence-name"),
            ),
        ],
    )?;
    Ok(())
}
