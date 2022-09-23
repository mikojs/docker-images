use std::env;

pub fn main(image_name: &str, env_name: &str, values: Vec<&str>) -> String {
    let docker_env_name = format!(
        "DOCKER_{}_VERSION",
        env_name.to_uppercase(),
    );

    if let Ok(env) = env::var(&docker_env_name) {
        return format!("{}:{}", image_name, env);
    }

    for value in values {
        if !value.is_empty() {
            return format!("{}:{}", image_name, value);
        }
    } 

    format!("{}:alpine", image_name)
}
