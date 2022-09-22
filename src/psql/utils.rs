use std::process;

use inquire::Confirm;

pub fn check_db_url(db_url: &str) -> bool {
    let message = format!("Use `{}`. Do you want to continue or not:", db_url);
    let result = Confirm::new(&message)
        .prompt();

    if let Ok(false) = result {
        process::exit(0);
    }

    true
}
