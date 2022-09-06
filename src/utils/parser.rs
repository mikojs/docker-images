use std::env;

pub fn print_env_value(name: &str) {
    if let Ok(value) = env::var(format!("DOCKER_{}_VERSION", name.to_uppercase())) {
        println!("{}", value);
    }
}
