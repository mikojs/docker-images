use std::env;
use std::process::Command;

#[allow(dead_code)]
#[path = "../utils/sub_process.rs"] mod sub_process;

fn shift_args(args: &mut Vec<String>) -> String {
    let command = args[0].clone();

    args.remove(0);
    command
}

fn main() {
    let mut args: Vec<String> = env::args()
        .collect();

    shift_args(&mut args);

    let mut main_args = shellwords::split(
      &shift_args(&mut args),
    )
      .expect("Couldn't get the commands");
    let custom_command = shift_args(&mut args);

    match Command::new(&custom_command).output() {
        Ok(_) => sub_process::exec(
            &custom_command,
            args
                .iter()
                .map(AsRef::as_ref)
                .collect(),
        ),
        Err(_) => sub_process::exec(
            &shift_args(&mut main_args),
            main_args
                .iter()
                .map(AsRef::as_ref)
                .collect(),
        ),
    }
}
