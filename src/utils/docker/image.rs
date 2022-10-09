use std::env;

use regex::Regex;

use crate::utils::{Error, ErrorKind};

pub const NAME_PATTERN: &str = r".+:<.+>";

fn get_version(versions: Vec<&str>) -> Result<String, Error> {
    let env_name_regex = Regex::new(r"DOCKER_.+_VERSION")?;

    for version in versions {
        if env_name_regex.is_match(version) {
            if let Ok(env) = env::var(version) {
                return Ok(env);
            }
        } else if !version.is_empty() {
            return Ok(version.to_string());
        }
    }

    Ok("alpine".to_string())
}

pub fn name(arg: &str) -> Result<String, Error> {
    let data: Vec<&str> = arg.split(":")
        .collect();

    if data.len() != 2 {
        return Err(
            Error::new(
                ErrorKind::Custom,
                format!("Couldn't parse {}", arg),
            ),
        );
    }

    let versions_str = data[1]
        .replace("<", "")
        .replace(">", "");
    let versions: Vec<&str> = versions_str
        .split("|")
        .collect();

    if versions.len() == 0 {
        return Err(
            Error::new(
                ErrorKind::Custom,
                format!("Couldn't find any version from {}", arg),
            ),
        );
    }

    let default_version = versions[versions.len() - 1];
    let version = get_version(versions)?;
    let image = format!("{}:{}", data[0], version);

    if version != default_version {
        println!("Custom Image: `{}`", image);
    }

    Ok(image)
}

#[test]
fn check_image_name() -> Result<(), Error> {
    let testings = vec![
        "alpine:latest",
        "alpine:<DOCKER_NOT_ENV_VERSION|latest>",
        "alpine:<DOCKER_ALPINE_VERSION|latest>",
    ];

    env::set_var("DOCKER_ALPINE_VERSION", "latest");
    for testing in testings {
        assert_eq!(name(testing)?, "alpine:latest");
    }
    Ok(())
}
