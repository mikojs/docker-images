use std::env;

#[allow(dead_code)]
#[path = "../utils/sub_process.rs"] mod sub_process;

fn get_command_help(command_name: &str) -> String {
    let command_file_path = env::current_exe()
        .expect("Couldn't get the current file path")
        .parent()
        .expect("Couldn't get the parent folder")
        .join(command_name);

    sub_process::exec_result(
        &command_file_path
            .display()
            .to_string(),
        vec!["--help"],
    )
}

fn main() {
    let command_names = vec!["ddocker", "code", "node-parser"];

    for command_name in command_names {
        println!("{}", get_command_help(command_name));
    }
}
