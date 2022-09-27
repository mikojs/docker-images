use std::env;
use std::process;

use inquire::Confirm;

pub struct Database {
    name: String,
    url: String,
}

impl Database {
    pub fn new(name: String) -> Database {
        let env_name = format!(
            "{}_DB_URL",
            name
                .replace("-", "_")
                .to_uppercase(),
        );

        if let Ok(url) = env::var(&env_name) {
            return Database {
                name: name,
                url: url,
            };
        }

        eprint!(
            "`{}` isn't in the environment variables.",
            env_name,
        );
        process::exit(1);
    }

    fn is_protected(&self) -> bool {
        if let Ok(not_protected_names) = env::var("NOT_PROTECTED_DBS") {
            return not_protected_names
                .split(",")
                .find(|&x| x == self.name)
                .is_none();
        }

        true
    }

    pub fn url<'a>(&'a self, danger_command: bool) -> &'a str {
        if danger_command && self.is_protected() {
            eprint!("The `{}` database is protected", &self.name);
            process::exit(1);
        }

        let message = format!("Use `{}`. Do you want to continue or not:", &self.url);
        let result = match Confirm::new(&message).prompt() {
            Ok(true) => true,
            _ => false,
        };

        if !result {
            process::exit(0);
        }

        &self.url
    }
}
