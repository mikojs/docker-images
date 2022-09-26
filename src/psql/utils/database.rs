use std::env;
use std::process;

pub struct Database {
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
                url: url,
            };
        }

        eprint!(
            "`{}` isn't in the environment variables.",
            env_name,
        );
        process::exit(1);
    }

    pub fn url<'a>(&'a self) -> &'a str {
        &self.url
    }
}
