use std::env;

use clap::{crate_version, Command};

#[path = "../psql/main.rs"] mod psql;

fn main() {
    let mut app = psql::command(
        Command::new("dpsql")
            .version(crate_version!())
            .about("Use psql command in the docker container")
    );

    for db_name in psql::get_db_names() {
        app = app.subcommand(
            psql::command(
                Command::new(&db_name)
                    .about("Database")
            ),
        );
    }

    let matches = app.get_matches();

    if let Some((sub_command, sub_matches)) = matches.subcommand() {
        for db_name in psql::get_db_names() {
            if sub_command == db_name {
                psql::execute(sub_matches, sub_command);
                return;
            }
        }
    }

    psql::execute(&matches, "default");
}
