use clap::Command;

#[path = "./utils/sub_process.rs"] mod sub_process;

pub const NAME: &str = "rm";

pub fn command() -> Command<'static> {
    Command::new("rm")
        .about("Find the all ids of the stopped containers and remove them")
}

pub fn execute() {
    let args = ["ps", "-aqf", "status=exited"];
    let stdout = sub_process::exec_result("docker", &args);
    let ids = stdout.split("\n");

    for id in ids {
        println!("{}", id);
    }
}
