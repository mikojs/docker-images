use std::process;
use std::process::Command;

const FAIL_TO_START: &str = "command failed to start";

pub fn exec(command: &str, args: Vec<&str>) {
    let status = Command::new(command)
        .args(args)
        .status()
        .expect(FAIL_TO_START);

    if !status.success() {
        process::exit(1);
    }
}

pub fn exec_result(command: &str, args: Vec<&str>) -> String {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect(FAIL_TO_START);

    String::from_utf8(output.stdout)
        .unwrap()
}
