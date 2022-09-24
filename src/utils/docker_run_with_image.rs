use std::env;

use regex::Regex;

#[path = "./docker_run.rs"] mod docker_run;

fn get_version(values: Vec<&str>) -> String {
    let env_name_regex = Regex::new(r"DOCKER_.+_VERSION")
        .unwrap();

    for value in values {
        if env_name_regex.is_match(value) {
            if let Ok(env) = env::var(value) {
                return env;
            }
        } else if !value.is_empty() {
            return value.to_string();
        }
    } 

    "alpine".to_string()
}

pub fn main(image_name: &str, values: Vec<&str>, args: Vec<&str>) {
    let last_version = values[values.len() - 1];
    let version = get_version(values);
    let docker_image = format!("{}:{}", image_name, &version);

    if version != last_version && version != "alpine" {
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
