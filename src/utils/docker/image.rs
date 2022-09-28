use std::env;
use std::process;

use regex::Regex;

pub const NAME_PATTERN: &str = r".+:<.+>";

fn get_version(versions: Vec<&str>) -> String {
    let env_name_regex = Regex::new(r"DOCKER_.+_VERSION")
        .unwrap();

    for version in versions {
        if env_name_regex.is_match(version) {
            if let Ok(env) = env::var(version) {
                return env;
            }
        } else if !version.is_empty() {
            return version.to_string();
        }
    }

    "alpine".to_string()
}

pub fn name(arg: &str) -> String {
    let data: Vec<&str> = arg.split(":")
        .collect();

    if data.len() != 2 {
        eprintln!("Couldn't parse {}", arg);
        process::exit(1);
    }

    let versions_str = data[1]
        .replace("<", "")
        .replace(">", "");
    let versions: Vec<&str> = versions_str
        .split("|")
        .collect();

    if versions.len() == 0 {
        eprintln!("Couldn't parse {}", arg);
        process::exit(1);
    }

    let default_version = versions[versions.len() - 1];
    let version = get_version(versions);
    let image = format!("{}:{}", data[0], version);

    if version != default_version {
        println!("Custom Image: `{}`", image);
    }

    image
}
