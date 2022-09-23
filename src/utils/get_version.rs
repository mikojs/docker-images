use std::env;

pub fn main(env_name: &str, values: Vec<&str>) -> String {
    let docker_env_name = format!(
        "DOCKER_{}_VERSION",
        env_name.to_uppercase(),
    );

    if let Ok(env) = env::var(&docker_env_name) {
        return env;
    }

    for value in values {
        if !value.is_empty() {
            return value.to_string();
        }
    } 

    return "alpine".to_string();
}
