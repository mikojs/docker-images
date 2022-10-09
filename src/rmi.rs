use clap::Command;

use crate::utils::{Error, sub_process};

pub fn command() -> Command<'static> {
    Command::new("rmi")
        .about("Find the all ids of the none-images and remove them")
}

pub fn execute() -> Result<(), Error> {
    let stdout = sub_process::exec_result(
        "docker",
        vec!["images", "-qf", "dangling=true"],
    )?;
    let ids: Vec<&str> = stdout.split("\n")
        .filter(|x| !x.is_empty())
        .collect();

    if ids.len() == 0 {
        println!("No none-images need to be removed.");
        return Ok(());
    }

    sub_process::exec(
        "docker",
        [vec!["rmi"], ids]
            .concat(),
    )?;
    Ok(())
}
