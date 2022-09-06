use std::process::Command;

const FAIL_TO_START: &str = "command failed to start";

pub fn exec(command: &str, args: &[&str]) {
    let status = Command::new(command)
        .args(args)
        .status()
        .expect(FAIL_TO_START);

    assert!(status.success());
}

pub fn exec_result(command: &str, args: &[&str]) -> String {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect(FAIL_TO_START);

    String::from_utf8(output.stdout)
        .unwrap()
}

pub fn exec_result_without_not_found_command(command: &str) -> String {
    let result: String;

    match Command::new(command).output() {
        Ok(command) => {
            result = String::from_utf8(command.stdout)
                .unwrap()
        },
        Err(_) => {
            result = "".to_string()
        },
    }

    result
}
