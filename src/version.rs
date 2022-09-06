use std::env;

use clap::{Command, ArgMatches};

#[allow(dead_code)]
#[path = "./utils/args.rs"] mod args;

pub fn command() -> Command<'static> {
    Command::new("version")
        .about(r#"Use this command to parse the docker version from the env variables
The arguments could be the env variable names or the parser names"#)
        .arg(args::set_proxy_arg())
}

pub fn execute(sub_matches: &ArgMatches) {
    let names: Vec<&str> = sub_matches
        .values_of("args")
        .unwrap()
        .collect();

    for name in names {
        if let Ok(value) = env::var(format!("DOCKER_{}_VERSION", name.to_uppercase())) {
            println!("{}", value);
            return;
        }
    }
}
