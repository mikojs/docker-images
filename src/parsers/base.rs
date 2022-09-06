use std::env;

use clap::{crate_version, Command, Arg};

#[path = "../utils/parser.rs"] mod parser;

fn cli() -> Command<'static> {
    Command::new("base-parser")
        .version(crate_version!())
        .about("This command would parse the value for the docker image")
        .arg(
            Arg::new("name")
                .required(true)
        )
}

fn main() {
    parser::print_env_value(
        &cli()
            .get_matches()
            .value_of("name")
            .expect("Couldn't parse the name in the args")
            .to_string(),
    );
}
