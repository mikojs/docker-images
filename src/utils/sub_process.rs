use std::process;
use std::process::Command;

use crate::utils::{Error, ErrorKind};

pub fn command_exist(command: &str) -> bool {
    match process::Command::new(command).output() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn exec(command: &str, args: Vec<&str>) -> Result<(), Error> {
    if !command_exist(command) {
        return Err(Error::new(
            ErrorKind::Custom,
            format!("Couldn't find the command: {}", command),
        ));
    }

    if !Command::new(command).args(args).status()?.success() {
        return Err(Error::new(
            ErrorKind::CommandFail,
            "Run command fails".to_string(),
        ));
    }
    Ok(())
}

pub fn exec_result(command: &str, args: Vec<&str>) -> Result<String, Error> {
    if !command_exist(command) {
        return Err(Error::new(
            ErrorKind::Custom,
            format!("Couldn't find the command: {}", command),
        ));
    }

    Ok(String::from_utf8(
        Command::new(command).args(args).output()?.stdout,
    )?)
}
