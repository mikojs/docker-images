use std::process;
use std::process::Command;
use std::io::Error;

pub fn exec(command: &str, args: Vec<&str>) -> Result<(), Error> {
    let status = Command::new(command)
        .args(args)
        .status()?;

    if !status.success() {
        process::exit(1);
    }
    Ok(())
}

pub fn exec_result(command: &str, args: Vec<&str>) -> Result<String, Error> {
    let output = Command::new(command)
        .args(args)
        .output()?;

    Ok(
        String::from_utf8(output.stdout)
            .unwrap()
    )
}
