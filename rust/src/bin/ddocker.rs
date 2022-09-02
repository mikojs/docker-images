use clap::{crate_version, Command};

#[path = "../rm.rs"] mod rm;
#[path = "../rmi.rs"] mod rmi;

fn cli() -> Command<'static> {
    return Command::new("ddocker")
        .version(crate_version!())
        .about("Some docker commands are used in a docker container")
        .subcommand_required(true)
        .subcommand(rm::command())
        .subcommand(rmi::command())
}

fn main() {
    match cli().get_matches().subcommand() {
        Some(("rm", _)) => rm::execute(),
        Some(("rmi", _)) => rmi::execute(),
        _ => unreachable!(),
    }
}
