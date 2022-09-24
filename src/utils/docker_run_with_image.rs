use std::env;

use regex::Regex;

#[path = "./docker_run.rs"] mod docker_run;

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

pub fn main(image_name: &str, versions: Vec<&str>, args: Vec<&str>) {
    let mut default_version = "alpine";

    if versions.len() != 0 {
        default_version = versions[versions.len() - 1];
    }

    let default_env: &str = &format!("DOCKER_{}_VERSION", image_name.to_uppercase());
    let version = get_version(
        [
            vec![default_env],
            versions,
        ]
            .concat(),
    );
    let docker_image = format!("{}:{}", image_name, &version);

    if version != default_version {
        println!("Custom Image: `{}`", docker_image);
    }

    docker_run::main(
        [
            vec!["-it", "--rm", &docker_image],
            args,
        ]
            .concat(),
    );
}
