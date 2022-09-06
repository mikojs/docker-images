use clap::{Command, Arg, ArgMatches};

#[path = "./utils/parser.rs"] mod parser;

pub fn command() -> Command<'static> {
    Command::new("version")
        .about("Use command to parse the docker version from the env variables")
        .arg(
            Arg::new("name")
                .required(true)
        )
}

pub fn execute(sub_matches: &ArgMatches) {
    parser::print_env_value(
        &sub_matches
            .value_of("name")
            .expect("Couldn't parse the name in the args")
            .to_string(),
    );
}
