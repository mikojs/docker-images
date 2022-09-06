use clap::Command;

#[allow(dead_code)]
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
        return println!("No none-images need to be removed.");
    }

    sub_process::exec(
        "docker",
        [vec!["rmi"], ids]
            .concat()
            .as_slice(),
    );
}
