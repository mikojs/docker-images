use clap::Command;

#[path = "./utils/sub_process.rs"] mod sub_process;

pub fn command() -> Command<'static> {
    Command::new("rmi")
        .about("Find the all ids of the none-images and remove them")
}

pub fn execute() {
    let stdout = sub_process::exec_result(
        "docker",
        &["images", "-qf", "dangling=true"],
    );
    let ids: Vec<&str> = stdout.split("\n")
        .filter(|x| !x.is_empty())
        .collect();

    if ids.len() == 0 {
        println!("No none-images need to be removed.");
        return;
    }

    let status = sub_process::exec(
        "docker",
        [vec!["rmi"], ids].concat().as_slice(),
    );

    assert!(status.success());
}
