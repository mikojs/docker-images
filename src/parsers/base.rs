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
        .value_of("name");

    println!("{:?}", name);
}
