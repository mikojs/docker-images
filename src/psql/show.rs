use clap::Command;

use crate::psql::utils::Database;

pub fn command() -> Command<'static> {
    Command::new("show")
        .about("Show the database url")
}

pub fn execute(mut db: Database) {
    println!("{}", db.url(false));
}
