use clap::Command;

#[path = "./utils/sub_process.rs"] mod sub_process;

pub fn command() -> Command<'static> {
    Command::new("rm")
        .about("Find the all ids of the stopped containers and remove them")
}

pub fn execute() {
    let stdout = sub_process::exec_result(
        "docker",
        &["ps", "-aqf", "status=exited"],
    );
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
