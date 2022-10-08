use std::io::Error;

use clap::Command;

use crate::utils::sub_process;

pub fn command() -> Command<'static> {
    Command::new("rm")
        .about("Find the all ids of the stopped containers and remove them")
}

pub fn execute() -> Result<(), Error> {
    let stdout = sub_process::exec_result(
        "docker",
        vec!["ps", "-aq", "-f", "status=exited", "-f", "status=created"],
    )?;
    let ids: Vec<&str> = stdout.split("\n")
        .filter(|x| !x.is_empty())
        .collect();

    if ids.len() == 0 {
        println!("No containers need to be removed.");
        return Ok(());
    }

    sub_process::exec(
        "docker",
        [vec!["rm"], ids]
            .concat(),
    );
    Ok(())
}
