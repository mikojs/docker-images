use std::env;
use std::process;

use inquire::Confirm;

fn is_protected_db(db_name: &str) -> bool {
    if let Ok(not_protected_db_names_str) = env::var("NOT_PROTECTED_DBS") {
        let not_protected_db_names: Vec<&str> = not_protected_db_names_str
            .split(",")
            .filter(|x| !x.is_empty())
            .collect();

        println!("{:?}", not_protected_db_names);
    }

    true
}

pub fn main(db_name: &str, db_url: &str, skip_protected_db_checking: bool) -> bool {
    if !skip_protected_db_checking && is_protected_db(db_name) {
        eprint!("The `{}` database is protected", db_name);
        process::exit(0);
    }

    let message = format!("Use `{}`. Do you want to continue or not:", db_url);
    let result = Confirm::new(&message)
        .prompt();

    if let Ok(false) = result {
        process::exit(0);
    }

    true
}
