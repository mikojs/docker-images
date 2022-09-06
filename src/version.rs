use clap::{Command, Arg, ArgMatches};

#[path = "./utils/parser.rs"] mod parser;

pub fn command() -> Command<'static> {
    Command::new("version")
        .about("Use this command to parse the docker version from the env variables")
        .arg(
            Arg::new("name")
                .required(true)
        )
}

pub fn execute(sub_matches: &ArgMatches) {
    println!(
        "{}",
        parser::get_env_value(
            &sub_matches
                .value_of("name")
                .expect("Couldn't parse the name from the arguments")
                .to_string(),
        ),
    );
}
