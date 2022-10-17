use clap::{crate_version, Command};

use docker_images::utils::Error;
use docker_images::docker::{exec, name, rm, rmi, run, volume};

fn main() -> Result<(), Error> {
    let matches = Command::new("ddocker")
        .version(crate_version!())
        .about("Some docker commands are used in a docker container")
        .subcommand_required(true)
        .subcommand(name::command())
        .subcommand(run::command())
        .subcommand(exec::command())
        .subcommand(rm::command())
        .subcommand(rmi::command())
        .subcommand(volume::command())
        .get_matches();

    match matches.subcommand() {
        Some(("name", _)) => name::execute()?,
        Some(("run", sub_matches)) => run::execute(sub_matches)?,
        Some(("exec", sub_matches)) => exec::execute(sub_matches)?,
        Some(("rm", _)) => rm::execute()?,
        Some(("rmi", _)) => rmi::execute()?,
        Some(("volume", sub_matches)) => volume::execute(sub_matches)?,
        _ => unreachable!(),
    };
    Ok(())
}
