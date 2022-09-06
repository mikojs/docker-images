use std::env;

pub fn get_env_value(name: &str) -> String {
    if let Ok(value) = env::var(format!("DOCKER_{}_VERSION", name.to_uppercase())) {
        return value;
    }

    "".to_string()
}
