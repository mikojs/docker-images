use std::process::{ExitStatus, Command};

const FAIL_TO_START: &str = "command failed to start";

pub fn exec(command: &str, args: &[&str]) -> ExitStatus {
    return Command::new(command)
        .args(args)
        .status()
        .expect(FAIL_TO_START);
}

pub fn exec_result(command: &str, args: &[&str]) -> String {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect(FAIL_TO_START);

    return String::from_utf8(output.stdout)
        .unwrap();
}