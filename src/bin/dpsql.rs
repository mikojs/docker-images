use std::env;
use std::io::Error;

use clap::{crate_version, Command};

use docker_images::psql;

fn main() -> Result<(), Error> {
    let mut app = psql::command(
        Command::new("dpsql")
            .version(crate_version!())
            .about("Use psql command in the docker container")
    );

    for db_name in psql::get_db_names() {
        if db_name != "default" {
            app = app.subcommand(
                psql::command(
                    Command::new(&db_name)
                        .about("Database name")
                ),
            );
        }
    }

    let matches = app.get_matches();

    if let Some((sub_command, sub_matches)) = matches.subcommand() {
        for db_name in psql::get_db_names() {
            if sub_command == db_name {
                psql::execute(sub_matches, sub_command)?;
                return Ok(());
            }
        }
    }

    psql::execute(&matches, "default")?;
    Ok(())
}
