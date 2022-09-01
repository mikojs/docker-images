use std::process::Command;

pub fn exec(command: &str, args: &[&str]) {
    Command::new(command)
        .args(args)
        .spawn()
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
