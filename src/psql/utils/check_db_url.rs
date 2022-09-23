use std::process;

use inquire::Confirm;

pub fn main(db_url: &str) -> bool {
    let message = format!("Use `{}`. Do you want to continue or not:", db_url);
    let result = Confirm::new(&message)
        .prompt();

    if let Ok(false) = result {
        process::exit(0);
    }

    true
}
