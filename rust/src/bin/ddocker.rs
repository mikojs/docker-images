use clap::{crate_version, Command};

fn cli() -> Command<'static> {
    Command::new("ddocker")
        .version(crate_version!())
        .about("Some docker commands are used in a docker container")
        .subcommand_required(true)
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        _ => unreachable!(),
    }
}
