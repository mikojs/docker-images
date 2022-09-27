use std::fs;
use std::fmt;
use std::env;
use std::process;

use inquire::Confirm;
use regex::Regex;

pub struct Database {
    name: String,
    url: String,
}

impl fmt::Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)
    }
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

    fn protected_error(&self) {
        eprint!("The `{}` database is protected", &self.name);
        process::exit(1);
    }

    pub fn url<'a>(&'a self, danger_command: bool) -> &'a str {
        if danger_command && self.is_protected() {
            self.protected_error();
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

    pub fn check_sql<'a>(&'a self, args: Vec<&'a str>) -> Vec<&'a str> {
        let keyword_regexs = vec![
            Regex::new(r"INSERT"),
            Regex::new(r"UPDATE"),
            Regex::new(r"DELETE"),
            Regex::new(r"ALTER"),
            Regex::new(r"TRUNCATE"),
        ];

        for arg in args.iter() {
            for keyword_regex in &keyword_regexs {
                let mut content = arg.to_string();

                if Regex::new(r"\.sql$").unwrap().is_match(&arg) {
                    content = match fs::read_to_string(arg) {
                        Ok(new_content) => new_content,
                        _ => content,
                    }
                }

                if keyword_regex.as_ref().unwrap().is_match(&content.to_uppercase()) && self.is_protected() {
                    self.protected_error();
                }
            }
        }

        args
    }
}
