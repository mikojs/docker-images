use std::process;
use clap::Command;

pub const NAME: &str = "rm";

pub fn command() -> Command<'static> {
    Command::new("rm")
        .about("Find the all ids of the stopped containers and remove them")
}

pub fn execute() {
    let output = process::Command::new("docker")
        .arg("ps")
        .arg("-aqf")
        .arg("status=exited")
        .output()
        .expect("command failed to start");
    let stdout = String::from_utf8(output.stdout)
        .unwrap();
    let ids = stdout.split("\n");

    for id in ids {
        println!("{}", id);
    }
}
