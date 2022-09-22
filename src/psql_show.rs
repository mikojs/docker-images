use std::env;

use clap::{Command};

pub fn command() -> Command<'static> {
    Command::new("show")
        .about("Show the database url")
}

pub fn execute(db_name: &str) -> String {
    let db_env_name = format!(
        "{}_DB_URL",
        db_name.to_uppercase(),
    );

    if let Ok(db_url) = env::var(&db_env_name) {
        return db_url;
    }

    "".to_string()
}
