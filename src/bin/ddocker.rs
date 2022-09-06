use clap::{crate_version, Command};

#[path = "../name.rs"] mod name;
#[path = "../run.rs"] mod run;
#[path = "../exec.rs"] mod exec;
#[path = "../rm.rs"] mod rm;
#[path = "../rmi.rs"] mod rmi;
#[path = "../version.rs"] mod version;

fn cli() -> Command<'static> {
    Command::new("ddocker")
        .version(crate_version!())
        .about("Some docker commands are used in a docker container")
        .subcommand_required(true)
        .subcommand(name::command())
        .subcommand(run::command())
        .subcommand(exec::command())
        .subcommand(rm::command())
        .subcommand(rmi::command())
        .subcommand(version::command())
}

fn main() {
    match cli().get_matches().subcommand() {
        Some(("name", _)) => name::execute(),
        Some(("run", sub_matches)) => run::execute(sub_matches),
        Some(("exec", sub_matches)) => exec::execute(sub_matches),
        Some(("rm", _)) => rm::execute(),
        Some(("rmi", _)) => rmi::execute(),
        Some(("version", sub_matches)) => version::execute(sub_matches),
        _ => unreachable!(),
    }
}
