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

    let status = Command::new(command).args(args).status()?;

    if !status.success() {
        return Err(Error::new(
            ErrorKind::CommandFail,
            "Run command fails".to_string(),
        ));
    }
    Ok(())
}

pub fn exec_result(command: &str, args: Vec<&str>) -> Result<String, Error> {
    let output = Command::new(command).args(args).output()?;

    Ok(String::from_utf8(output.stdout)?)
}
