use std::fs;
use std::fmt;
use std::env;
use std::process;

use inquire::Confirm;
use regex::Regex;

use crate::utils::docker;

pub struct Database {
    name: String,
    url: String,
    is_protected: bool,
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
            let is_protected = match env::var("NOT_PROTECTED_DBS") {
                Ok(env) => env
                    .split(",")
                    .find(|&x| x == name)
                    .is_none(),
                _ => true,
            };

            return Database {
                name: name,
                url: url,
                is_protected: is_protected,
            };
        }

        eprint!(
            "`{}` isn't in the environment variables.",
            env_name,
        );
        process::exit(1);
    }

    pub fn url<'a>(&'a self, danger_command: bool) -> &'a str {
        if danger_command {
            if self.is_protected {
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
        } else {
            println!("DB url: {}", &self.url);
        }

        &self.url
    }

    fn is_danger_arg(&self, arg: &str) -> bool {
        let keyword_regexs = vec![
            Regex::new(r"INSERT[ \n]"),
            Regex::new(r"CREATE[ \n]"),
            Regex::new(r"UPDATE[ \n]"),
            Regex::new(r"DELETE[ \n]"),
            Regex::new(r"ALTER[ \n]"),
            Regex::new(r"TRUNCATE[ \n]"),
            Regex::new(r"DROP[ \n]"),
        ];

        for keyword_regex in &keyword_regexs {
            let mut content = arg.to_string();

            if Regex::new(r"\.sql$").unwrap().is_match(&arg) {
                content = match fs::read_to_string(arg) {
                    Ok(new_content) => new_content,
                    _ => content,
                }
            }

            if keyword_regex.as_ref().unwrap().is_match(&content.to_uppercase()) && self.is_protected {
                return true
            }
        }

        false
    }

    pub fn check_sql<'a>(&'a self, args: Vec<&'a str>) -> Vec<&'a str> {
        for arg in args.iter() {
            if self.is_danger_arg(arg) {
                eprint!("The `{}` database is protected", &self.name);
                process::exit(1);
            }
        }

        args
    }

    pub fn run(&self, args: Vec<&str>) {
        let mut is_danger = false;

        for arg in args.iter() {
            if self.is_danger_arg(arg) {
                is_danger = true;
                break;
            }
        }

        if is_danger {
            if self.is_protected {
                eprint!("The `{}` database is protected", &self.name);
                process::exit(1);
            }

            let message = format!("Use `{}`. Do you want to continue or not:", &self.url);
            let result = match Confirm::new(&message).prompt() {
                Ok(true) => true,
                _ => false,
            };

            if !result {
                return;
            }
        } else {
            println!("DB url: {}", &self.url);
        }

        docker::run(
            [
                vec![
                    "-it",
                    "--rm",
                    "postgres:<DOCKER_POSTGRES_VERSION|alpine>",
                ],
                args,
            ]
                .concat(),
        );
    }
}

#[test]
fn set_testing_env() {
    env::set_var("DEFAULT_DB_URL", "test");
    env::set_var("PROTECTED_DB_URL", "test");
    env::set_var("NOT_PROTECTED_DBS", "default,foo,bar");
}

#[test]
fn check_url() {
    set_testing_env();
    assert_eq!(Database::new("default".to_string()).url, "test");
}

#[test]
fn check_db_is_protected() {
    set_testing_env();
    assert_eq!(Database::new("default".to_string()).is_protected, false);
    assert_eq!(Database::new("protected".to_string()).is_protected, true);
}

#[test]
fn check_sql() {
    let testing_sql_file_path = "./testing.sql";
    let testings = vec![
        "CREATE ",
        r#"CREATE
"#,
        "DELETE ",
        r#"DELETE
"#,
    ];

    set_testing_env();
    for testing in testings {
        fs::write(testing_sql_file_path, testing)
            .expect("Couldn't create the testing file");

        assert_eq!(
            Database::new("protected".to_string())
                .is_danger_arg(testing),
            true,
        );
        assert_eq!(
            Database::new("protected".to_string())
                .is_danger_arg(testing_sql_file_path),
            true,
        );

        fs::remove_file(testing_sql_file_path)
            .expect("Couldn't remove the testing file");
    }
}
