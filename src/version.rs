use std::env;

use clap::{Command, ArgMatches};
use regex::Regex;

#[allow(dead_code)]
#[path = "./utils/args.rs"] mod args;
#[allow(dead_code)]
#[path = "./utils/sub_process.rs"] mod sub_process;

pub fn command() -> Command<'static> {
    Command::new("version")
        .about(r#"Use this command to parse the docker version from the env variables
The arguments could be the env variable names or the parser names, like `rust rust-parser`"#)
        .arg(args::set_proxy_arg())
}

pub fn execute(sub_matches: &ArgMatches) {
    let names: Vec<&str> = sub_matches
        .values_of("args")
        .unwrap()
        .collect();

    for name in names {
        let is_parser = Regex::new(r"-parser$")
            .unwrap()
            .is_match(&name);

        if is_parser {
            let version = sub_process::exec_result_without_not_found_command(&name);

            if !version.is_empty() {
                println!("{}", version);
                return;
            }
        }

        if let Ok(version) = env::var(format!("DOCKER_{}_VERSION", name.to_uppercase())) {
            println!("{}", version);
            return;
        }
    }
}
