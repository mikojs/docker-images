use std::fs;
use std::fmt;
use std::env;

use regex::Regex;

use crate::utils::{Error, ErrorKind, docker, prompt};

fn is_danger_arg(arg: &str) -> Result<bool, Error> {
    let keyword_regexs = vec![
        r"INSERT[ \n]",
        r"CREATE[ \n]",
        r"UPDATE[ \n]",
        r"DELETE[ \n]",
        r"ALTER[ \n]",
        r"TRUNCATE[ \n]",
        r"DROP[ \n]",
        r"PG_RESTORE",
        r"\\COPY .+ FROM '.+' WITH CSV",
    ];

    for keyword_regex in &keyword_regexs {
        let mut content = arg.to_string();

        if Regex::new(r"\.sql$")?.is_match(&arg) {
            content = match fs::read_to_string(arg) {
                Ok(new_content) => new_content,
                _ => content,
            }
        }

        if Regex::new(keyword_regex)?.is_match(&content.to_uppercase()) {
            return Ok(true)
        }
    }

    Ok(false)
}

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

impl Database {
    pub fn new(name: String) -> Result<Database, Error> {
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

            return Ok(
                Database {
                    name: name,
                    is_protected: is_protected,
                    url: url,
                }
            );
        }

        Err(
            Error::new(
                ErrorKind::Custom,
                format!("`{}` isn't in the environment variables.", env_name),
            ),
        )
    }

    pub fn run(&self, args: Vec<&str>) -> Result<(), Error> {
        let mut is_danger = false;

        for arg in args.iter() {
            if is_danger_arg(arg)? {
                is_danger = true;
                break;
            }
        }

        if is_danger {
            if self.is_protected {
                return Err(
                    Error::new(
                        ErrorKind::Custom,
                        format!("The `{}` database is protected", &self.name),
                    ),
                );
            }

            if !prompt(&format!("Use `{}`. Do you want to continue or not:", &self.url)) {
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
fn check_danger_args() -> Result<(), Error> {
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

    fn check_danger_arg(testing: &str, expected: bool) -> Result<(), Error> {
        let testing_sql_file_path = "./testing.sql";

        fs::write(testing_sql_file_path, testing)?;

        assert_eq!(is_danger_arg(testing)?, expected);
        assert_eq!(is_danger_arg(testing_sql_file_path)?, expected);

        fs::remove_file(testing_sql_file_path)?;
        Ok(())
    }


    for danger_testing in danger_testings {
        check_danger_arg(danger_testing, true)?;
    }
    for not_danger_testing in not_danger_testings {
        check_danger_arg(not_danger_testing, false)?;
    }
    Ok(())
}

#[test]
fn db_init() -> Result<(), Error> {
    env::set_var("DEFAULT_DB_URL", "test");
    env::set_var("PROTECTED_DB_URL", "test");
    env::set_var("NOT_PROTECTED_DBS", "default,foo,bar");

    assert_eq!(Database::new("default".to_string())?.url, "test");
    assert_eq!(Database::new("default".to_string())?.is_protected, false);
    assert_eq!(Database::new("protected".to_string())?.is_protected, true);
    Ok(())
}
