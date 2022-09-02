use clap::Command;

#[path = "./utils/sub_process.rs"] mod sub_process;

pub const NAME: &str = "rm";

pub fn command() -> Command<'static> {
    return Command::new("rm")
        .about("Find the all ids of the stopped containers and remove them")
}

pub fn execute() {
    let args = ["ps", "-aqf", "status=exited"];
    let stdout = sub_process::exec_result("docker", &args);
    let ids: Vec<&str> = stdout.split("\n")
        .filter(|x| !x.is_empty())
        .collect();

    if ids.len() == 0 {
        println!("No containers need to be removed.");
        return;
    }

    let status = sub_process::exec(
        "docker",
        [vec!["rm"], ids].concat().as_slice(),
    );

    assert!(status.success());
}
