use std::env;

use clap::{crate_version, Command, Arg};

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
    let matches = cli()
        .get_matches();
    let name = matches
        .value_of("name")
        .expect("Couldn't parse the name in the args")
        .to_string()
        .to_uppercase();

    if let Ok(value) = env::var(format!("DOCKER_{}_VERSION", name)) {
        println!("{:?}", value);
    }
}
