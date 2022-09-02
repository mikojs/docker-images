use std::process::{ExitStatus, Command};

pub fn exec(command: &str, args: &[&str]) -> ExitStatus {
    return Command::new(command)
        .args(args)
        .status()
        .expect("command failed to start");
}

pub fn exec_result(command: &str, args: &[&str]) -> String {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("command failed to start");

    return String::from_utf8(output.stdout)
        .unwrap();
}
