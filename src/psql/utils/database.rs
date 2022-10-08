use std::fs;
use std::fmt;
use std::env;
use std::process;
use std::io::Error;

use inquire::Confirm;
use regex::Regex;

use crate::utils::docker;

pub struct Database {
    name: String,
    is_protected: bool,
    pub url: String,
}

impl fmt::Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}

fn is_danger_arg(arg: &str) -> bool {
    let keyword_regexs = vec![
        Regex::new(r"INSERT[ \n]"),
        Regex::new(r"CREATE[ \n]"),
        Regex::new(r"UPDATE[ \n]"),
        Regex::new(r"DELETE[ \n]"),
        Regex::new(r"ALTER[ \n]"),
        Regex::new(r"TRUNCATE[ \n]"),
        Regex::new(r"DROP[ \n]"),
        Regex::new(r"PG_RESTORE"),
        Regex::new(r"\\COPY .+ FROM '.+' WITH CSV"),
    ];

    for keyword_regex in &keyword_regexs {
        let mut content = arg.to_string();

        if Regex::new(r"\.sql$").unwrap().is_match(&arg) {
            content = match fs::read_to_string(arg) {
                Ok(new_content) => new_content,
                _ => content,
            }
        }

        if keyword_regex.as_ref().unwrap().is_match(&content.to_uppercase()) {
            return true
        }
    }

    false
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
                is_protected: is_protected,
                url: url,
            };
        }

        eprint!(
            "`{}` isn't in the environment variables.",
            env_name,
        );
        process::exit(1);
    }

    pub fn run(&self, args: Vec<&str>) -> Result<(), Error> {
        let mut is_danger = false;

        for arg in args.iter() {
            if is_danger_arg(arg) {
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
                return Ok(());
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
        )?;
        Ok(())
    }
}

#[test]
fn check_danger_args() {
    let danger_testings = vec![
        "CREATE ",
        r#"CREATE
"#,
        "pg_restore",
        "\\copy test(key, value) FROM 'test.csv' WITH csv",
    ];
    let not_danger_testings = vec![
        "pg_dump",
        "\\copy (SELECT * FROM test) TO 'test.csv' WITH csv",
    ];

    fn check_danger_arg(testing: &str, expected: bool) {
        let testing_sql_file_path = "./testing.sql";

        fs::write(testing_sql_file_path, testing)
            .expect("Couldn't create the testing file");

        assert_eq!(is_danger_arg(testing), expected);
        assert_eq!(is_danger_arg(testing_sql_file_path), expected);

        fs::remove_file(testing_sql_file_path)
            .expect("Couldn't remove the testing file");
    }


    for danger_testing in danger_testings {
        check_danger_arg(danger_testing, true);
    }
    for not_danger_testing in not_danger_testings {
        check_danger_arg(not_danger_testing, false);
    }
}

#[test]
fn db_init() {
    env::set_var("DEFAULT_DB_URL", "test");
    env::set_var("PROTECTED_DB_URL", "test");
    env::set_var("NOT_PROTECTED_DBS", "default,foo,bar");

    assert_eq!(Database::new("default".to_string()).url, "test");
    assert_eq!(Database::new("default".to_string()).is_protected, false);
    assert_eq!(Database::new("protected".to_string()).is_protected, true);
}
