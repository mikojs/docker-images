use clap::{crate_version, Command};

#[path = "../rm.rs"] mod rm;

fn cli() -> Command<'static> {
    return Command::new("ddocker")
        .version(crate_version!())
        .about("Some docker commands are used in a docker container")
        .subcommand_required(true)
        .subcommand(rm::command())
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some((rm::NAME, _)) => rm::execute(),
        _ => unreachable!(),
    }
}
