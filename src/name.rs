use clap::Command;

use crate::utils::{Error, docker};

pub fn command() -> Command<'static> {
    Command::new("name")
        .about("Show the current container id")
}

pub fn execute() -> Result<(), Error> {
    println!("{}", docker::name()?);
    Ok(())
}
