use clap::{crate_version, Command};

#[path = "../run.rs"] mod run;
#[path = "../rm.rs"] mod rm;
#[path = "../rmi.rs"] mod rmi;

fn cli() -> Command<'static> {
    return Command::new("ddocker")
        .version(crate_version!())
        .about("Some docker commands are used in a docker container")
        .subcommand_required(true)
        .subcommand(run::command())
        .subcommand(rm::command())
        .subcommand(rmi::command())
}

fn main() {
    match cli().get_matches().subcommand() {
        Some(("run", _)) => rm::execute(),
        Some(("rm", _)) => rm::execute(),
        Some(("rmi", _)) => rmi::execute(),
        _ => unreachable!(),
    }
}
